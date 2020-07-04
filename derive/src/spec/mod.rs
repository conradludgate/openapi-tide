// use openapi_utils::{SpecExt, ReferenceOrExt};
use syn::{parse_quote, Item};
use inflector::Inflector;
use quote::format_ident;
use proc_macro2::Span;
use filetree::FileTree;

mod fields;
mod filetree;
mod schema;
mod path;

pub(crate) fn get<P>(filename: P) -> openapiv3::OpenAPI
where P: AsRef<std::path::Path> + Clone + std::fmt::Debug {
    let data = std::fs::read_to_string(filename.clone())
        .expect("OpenAPI file could not be read.");
    
    let ext = filename.as_ref().extension().unwrap();
    if ext == "json" {
        serde_json::from_str(&data).expect("Could not deserialize file as OpenAPI v3.0 json")
    } else {
        serde_yaml::from_str(&data).expect("Could not deserialize file as OpenAPI v3.0 yaml")
    }
}

pub(crate) fn convert(spec: openapiv3::OpenAPI) -> Vec<Item> {
    let mut items = vec![];

    // let mut schemas: indexmap::IndexMap<String, Item> = indexmap::IndexMap::new();
    // let mut schema_set: Set<openapiv3::Schema, String> = Set::new();

    // If any refs point to external files, add them to this queue
    // for processing at the end.
    // Each folder and file will be a module
    let mut ext_refs: FileTree = Default::default();
    
    if let Some(components) = spec.components {
        let schemas = components.schemas.into_iter().flat_map(|(name, schema)|
            match schema {
                openapiv3::ReferenceOr::Reference{reference} => {
                    let path = ext_refs.insert_ref(reference);
                    alias(name.to_class_case(), path, 2)
                },
                openapiv3::ReferenceOr::Item(schema) => {
                    schema::convert(name.to_class_case(), schema)
                }
            }
        );
        
        items.push(parse_quote!{
            pub mod components {
                pub mod schemas {
                    #(#schemas)*
                }
            }
        })
    }

    let (path_items, trait_fns, path_decls): (Vec<_>, Vec<_>, Vec<_>) = spec.paths.into_iter().flat_map(|(path, path_item)|
        match path_item {
            openapiv3::ReferenceOr::Reference{reference: _} => {
                // let path = ext_refs.insert_ref(reference);
                // alias(name.to_class_case(), path, 1)
                todo!("path refs")
            },
            openapiv3::ReferenceOr::Item(path_item) => {
                path::convert(&mut ext_refs, path, path_item)
            }
        }
    ).unzip();

    items.push(parse_quote!{
        mod paths {
            #(#path_items)*

            #[::async_trait::async_trait]
            trait Spec: Sized + Send + Sync + 'static {
                #(#trait_fns)*
            }
        }

        fn into_server(self) -> tide::Server<Self> {
            let mut app = tide::with_state(self);

            #(#path_decls)*
            
            app
        }
    });

    items
}

pub(crate) fn alias(name: String, path: syn::Path, nested: i32) -> Vec<Item> {
    let name_ident = format_ident!("{}", name);

    let supers = (0..nested).map(|_| syn::token::Super{span: Span::call_site()});

    vec![
        parse_quote!{
            type #name_ident = #(#supers::)*#path;
        }
    ]
}