use std::collections::HashMap;
use std::path::Path;
use proc_macro2::Span;

pub(crate) enum FileTree {
    Folder(HashMap<String, FileTree>),
    File(String),
}

impl Default for FileTree {
    fn default() -> Self {
        FileTree::Folder(HashMap::new())
    }
}

impl FileTree {
    pub fn insert(&mut self, path: String) {
        let sections: Vec<String> = Path::new(&path)
            .into_iter()
            .map(|s| s.to_str().unwrap().to_owned())
            .collect();

        let (file_name, dirs) = sections.split_last().unwrap();
        self.insert_slice(dirs, file_name.to_owned(), path);
    }

    fn insert_slice(&mut self, dirs: &[String], file_name: String, path: String) {
        let folder = match self {
            FileTree::Folder(folder) => folder,
            _ => panic!("cannot insert into file type in filetree"),
        };
        if dirs.len() == 0 {
            folder.insert(file_name, FileTree::File(path));
        } else {
            let (first, rest) = dirs.split_first().unwrap();
            let mut new_folder = FileTree::default();
            new_folder.insert_slice(rest, file_name, path);
            folder.insert(first.to_owned(), new_folder);
        }
    }

    pub fn insert_ref(&mut self, reference: String) -> syn::Path {
        let mid = reference.rfind('#').unwrap_or(reference.len());
        let (path, location) = reference.split_at(mid);

        eprintln!("path: {:?}, location: {:?}", path, location);

        let mut mods: Vec<syn::Ident> = Path::new(path)
            .with_extension("")
            .into_iter()
            .map(|s| s.to_str().unwrap())
            .map(|s| if s == ".." {
                syn::Ident::new("super", Span::call_site())
            } else {
                syn::Ident::new(s, Span::call_site())
            })
            .collect();

        eprintln!("mods: {:?}", mods);
        
        if mods.len() != 0 {
            self.insert(path.to_string());
        }

        if location.len() > 0 {
            mods.extend(Path::new(&location[2..])
                .into_iter()
                .map(|s| s.to_str().unwrap())
                .map(|s|
                    syn::Ident::new(s, Span::call_site())
                )
            );
        }

        eprintln!("mods: {:?}", mods);
            
        syn::parse_quote!{
            #(#mods)::*
        }
    }
}