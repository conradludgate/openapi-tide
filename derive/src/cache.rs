use std::path::Path;

use crate::attr::Attributes;

pub(crate) fn find(base_dir: &Path, attr: &Attributes) -> Option<()> {
    let path = base_dir
        .join(".openapi")
        .join(format!("{}.build", attr.spec_file.value()));
    // println!("{:?}", path);
    
    // std::fs::create_dir_all(path.parent().expect("get parent")).expect("create dir");
    // let mut file = std::fs::File::create(path).expect("create file");

    // use std::io::prelude::Write;
    // write!(file, "version: {}\nspec_file: {}", attr.version.value(), attr.spec_file.value()).expect("write file");

    std::fs::File::open(path).ok().map(|_| ())
}

pub(crate) fn update(base_dir: &Path, attr: &Attributes, _cache: ()) {
    let path = base_dir
        .join(".openapi")
        .join(format!("{}.build", attr.spec_file.value()));
    
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut file = std::fs::File::create(path).expect("create file");

    use std::io::prelude::Write;
    write!(file, "spec_file: {}", attr.spec_file.value()).unwrap();
}