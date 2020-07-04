use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::parse_quote;
use quote::{quote, ToTokens};
use inflector::Inflector;

pub(crate) fn convert(ext_refs: &mut super::FileTree, path: String, path_item: openapiv3::PathItem) -> Vec<(syn::ItemMod, syn::ItemFn, PathDecl)> {
    let mut routes = vec![];

    add_route(ext_refs, &mut routes, path.clone(), path_item.get, Method::Get);
    add_route(ext_refs, &mut routes, path.clone(), path_item.post, Method::Post);
    add_route(ext_refs, &mut routes, path.clone(), path_item.put, Method::Put);
    add_route(ext_refs, &mut routes, path.clone(), path_item.delete, Method::Delete);
    add_route(ext_refs, &mut routes, path.clone(), path_item.options, Method::Options);
    add_route(ext_refs, &mut routes, path.clone(), path_item.head, Method::Head);
    add_route(ext_refs, &mut routes, path.clone(), path_item.patch, Method::Patch);
    add_route(ext_refs, &mut routes, path.clone(), path_item.trace, Method::Trace);

    routes
}
enum Method {
    Get,
    Post,
    Put,
    Delete,
    Options,
    Head,
    Patch,
    Trace,
}

impl Method {
    pub fn str(&self) -> &str {
        match self {
            Get => "get",
            Post => "post",
            Put => "put",
            Delete => "delete",
            Options => "options",
            Head => "head",
            Patch => "patch",
            Trace => "trace",
        }
    }

    pub fn ident(&self) -> syn::Ident {
        syn::Ident::new(self.str(), Span::call_site())
    }
}

fn add_route(ext_refs: &mut super::FileTree, routes: &mut Vec<(syn::ItemMod, syn::ItemFn, PathDecl)>, path: String, op: Option<openapiv3::Operation>, method: Method) {
    let op = match op {
        Some(op) => op,
        None => return,
    };

    let operation_id = match op.operation_id {
        Some(operation_id) => operation_id.to_snake_case(),
        None => {
            (method.str().to_string() + &path
                .replace('/', "_")
                .replace('{', "_")
                .replace('}', "_")
            ).to_snake_case()
        }
    };

    let request: Option<syn::Type> = op.request_body.map(|rb|
        match rb {
            openapiv3::ReferenceOr::Reference{reference} => {
                // let path = ext_refs.insert_ref(reference);
                todo!("request ref")
            },
            openapiv3::ReferenceOr::Item(rb) => {
                // schema::convert(name.to_class_case(), schema)
                // todo!("unref request")
                // rb.content
                // use content field to 
                
            }
        }
    );

    // /// get name of server if some is set to true
    // /// POST: /bar
    // async fn maybeGetName(&self, body: Req, req: &tide::Request<Self>) -> tide::Result<(Resp, tide::StatusCode)>;


}

pub(crate) struct PathDecl {
    path: String,
    method: syn::Ident,
    request: Option<syn::Type>, // add content type which defines the decode fn
    params: Option<syn::Type>,
    handler: syn::Ident,
}

impl ToTokens for PathDecl {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            path,
            method,
            request,
            params,
            handler,
        } = self;

        let mut inputs = vec![];
        let mut decodes = vec![];
        if let Some(params) = params {
            decodes.push(quote!{
                let params = #params::decode(&mut req).await?
            });
            inputs.push(syn::Ident::new("params", Span::call_site()));
        }
        if let Some(request) = request {
            decodes.push(quote!{
                let body = #request::decode(&mut req).await?
            });
            inputs.push(syn::Ident::new("body", Span::call_site()));
        }

        tokens.extend(quote!{
            app.at(#path).#method(
                |mut req: ::tide::Request<Self>| async move {
                    #(#decodes;)*
                    let (resp, status) = req.state().#handler(#(#inputs,)* &req).await?;
                    Ok(::tide::Response::new(status).body_json(&resp)?)
                }
            )
        });
    }
}