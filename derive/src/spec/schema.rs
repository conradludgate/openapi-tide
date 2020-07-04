use syn::{parse_quote, Item};
use inflector::Inflector;
use quote::format_ident;
use proc_macro2::Span;
use super::fields::Named;

pub(crate) fn convert(name: String, schema: openapiv3::Schema) -> Vec<Item> {
    let attrs = vec![
        create_doc(schema.schema_data),
        // parse_quote! {
        //     #[derive(::serde::Deserialize, ::serde::Serialize, ::std::fmt::Debug)]
        // },
    ];
    
    let name_ident = format_ident!("{}", name);

    // eprintln!("ident: {:?}", name_ident);

    let mut items = vec![];

    match schema.schema_kind {
        openapiv3::SchemaKind::Type(t) => {
            match t {
                openapiv3::Type::String(s) => {
                    if s.enumeration.len() > 0 {
                        let names = s.enumeration;

                        let variants: Vec<syn::Variant> = names.iter()
                            .map(|name| {
                                let case = syn::Ident::new(&name.to_class_case(), Span::call_site());
                                parse_quote!{
                                    #case = #name
                                }
                            }).collect();

                        // println!("{:#?}", variants.iter().map(|v| v.to_token_stream()).collect::<Vec<proc_macro2::TokenStream>>());

                        items.push(parse_quote!{
                            #(#attrs)*
                            enum #name_ident {#(#variants,)*}
                        })
                    } else {
                        // TODO: custom serde for when you have max/min length
                        // or for when you have a format
                        // potentially add support for custom format types at attribute level
                        // eg `#[openapi(file, version, "url" => URL)]`
                        // where URL will be the type instead of string
                        items.push(parse_quote!{
                            #(#attrs)*
                            type #name_ident = ::std::string::String;
                        });
                    }
                },
                openapiv3::Type::Number(_) => {
                    items.push(parse_quote!{
                        #(#attrs)*
                        type #name_ident = f64;
                    });
                },
                openapiv3::Type::Integer(_) => {
                    items.push(parse_quote!{
                        #(#attrs)*
                        type #name_ident = i64;
                    });
                },
                openapiv3::Type::Object(o) => {
                    create_struct(o, name_ident, name, attrs, &mut items);
                },
                openapiv3::Type::Array(a) => {
                    create_array(a, name, attrs, &mut items);
                },
                openapiv3::Type::Boolean{} => {
                    items.push(parse_quote!{
                        #(#attrs)*
                        type #name_ident = bool;
                    });
                },
            }
        },
        openapiv3::SchemaKind::Any(_) => {},
        _ => (),
    };
    // eprintln!("ident2: {:?}", name_ident);

    items
}

fn schema_to_type(schema: openapiv3::Schema, field_name: String) -> (syn::Type, Vec<Item>) {
    let attrs = vec![
        create_doc(schema.schema_data),
        // parse_quote! {
        //     #[derive(::serde::Deserialize, ::serde::Serialize, ::std::fmt::Debug)]
        // },
    ];

    let name_ident = format_ident!("{}", field_name.to_class_case());

    let mut items = vec![];
    let ty: syn::Type = match schema.schema_kind {
        openapiv3::SchemaKind::Type(t) => {
            match t {
                openapiv3::Type::String(s) => {
                    if s.enumeration.len() > 0 {
                        // TODO: generate enum + name for the type
                        parse_quote!(::std::string::String)
                    } else {
                        parse_quote!(::std::string::String)
                    }
                },
                openapiv3::Type::Number(_) => parse_quote!(f64),
                openapiv3::Type::Integer(_) =>  parse_quote!(i64),
                openapiv3::Type::Object(o) => {
                    create_struct(o, name_ident, field_name, attrs, &mut items)
                },
                openapiv3::Type::Array(a) => {
                    create_array(a, field_name, attrs, &mut items)
                },
                openapiv3::Type::Boolean{} => parse_quote!(bool),
            }
        },
        openapiv3::SchemaKind::Any(_) => parse_quote!(Vec<u8>),
        _ => todo!("all/anyof/oneof. All => Struct of each type, Any => Vec of enum, One of => enum"),
    };

    (ty, items)
}

fn create_struct(
    o: openapiv3::ObjectType,
    ident: syn::Ident,
    name: String,
    attrs: Vec<syn::Attribute>,
    items: &mut Vec<Item>) -> syn::Type
{
    let mut fields: Vec<Named> = vec![];
    for (prop_name, ty) in o.properties {
        let ty = match ty {
            openapiv3::ReferenceOr::Reference{reference} => {
                let split: Vec<String> = reference.split('/').map(str::to_owned).collect();
                let ty = format_ident!("{}", split.last().unwrap().to_snake_case());
                parse_quote!(#ty)
            },
            openapiv3::ReferenceOr::Item(s) => {
                let (ty, its) = schema_to_type(*s, 
                    format!("{}_{}", name, prop_name).to_class_case()
                );
                items.extend(its);
                ty
            },
        };
        
        let field = format_ident!("{}", prop_name.to_snake_case());
        

        // Wrap in optional if not required
        if o.required.contains(&prop_name) {
            fields.push(parse_quote!{
                // #[serde(rename = #prop_name)]
                pub #field: #ty
            });
        } else {
            fields.push(parse_quote!{
                // #[serde(rename = #prop_name)]
                pub #field: ::std::option::Option<#ty>
            });
        };

        
    }

    items.push(parse_quote!{
        #(
            #attrs
        )*
        pub struct #ident {
            #(
                #fields,
            )*
        }
    });

    parse_quote!(#ident)
}

fn create_array(
    a: openapiv3::ArrayType,
    name: String,
    attrs: Vec<syn::Attribute>,
    items: &mut Vec<Item>) -> syn::Type
{
    let ty = match a.items {
        openapiv3::ReferenceOr::Reference{reference} => {
            let split: Vec<String> = reference.split('/').map(str::to_owned).collect();
            let ty = format_ident!("{}", split.last().unwrap().to_snake_case());
            parse_quote!(#ty)
        },
        openapiv3::ReferenceOr::Item(s) => {
            let (ty, its) = schema_to_type(*s, name.clone());
            items.extend(its);
            ty
        },
    };

    let name_ident = format_ident!("{}", name.to_plural().to_pascal_case());

    items.push(parse_quote!{
        #(
            #attrs
        )*
        type #name_ident = Vec<#ty>;
    });

    parse_quote!(#name_ident)
}

fn create_doc(data: openapiv3::SchemaData) -> syn::Attribute {
    let docs = vec![
        data.title,
        data.description,
        data.external_docs.map(|ed| 
            ed.description
                .map_or(String::new(), |d| d + " ")
                + &ed.url
        )
    ];
    let doc = docs
        .into_iter()
        .flatten()
        .map(|s| s.clone())
        .collect::<Vec<String>>()
        .join("\n\n");

    parse_quote! { #[doc = #doc] }
}