use proc_macro::TokenStream;
use syn::{parse_quote, parse_macro_input};
use quote::ToTokens;

mod cache;
mod attr;
mod spec;

use attr::Attributes;

#[proc_macro_attribute]
pub fn derive_openapi(attr: TokenStream, input: TokenStream) -> TokenStream {

    eprintln!("start");
    let attr = parse_macro_input!(attr as Attributes);

    eprintln!("attr");

    let dir = std::env::var("CARGO_MANIFEST_DIR").expect("manifest");
    let dir = std::path::Path::new(&dir);

    cache::find(dir, &attr);

    eprintln!("cache");

    let spec = spec::get(dir.join(&attr.spec_file.value()));

    eprintln!("spec");
    // eprintln!("{:#?}", spec);

    let version = spec.info.version.clone();

    eprintln!("version");

    let items = spec::convert(spec);

    eprintln!("items");

    // let spec = openapi::from_path(attr.spec_file.value());

    let mut module = parse_macro_input!(input as syn::ItemMod);

    // eprintln!("module");
    
    let mut contents = module.content.unwrap_or((Default::default(), vec![]));

    contents.1.extend(items);

    let version_const: syn::Item = parse_quote!{
        pub(super) const VERSION: &str = #version;
    };

    contents.1.push(version_const);

    module.content = Some(contents);

    cache::update(dir, &attr, ());

    // eprintln!("update cache");

    module.into_token_stream().into()
}

