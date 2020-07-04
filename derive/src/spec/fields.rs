
use quote::{ToTokens, TokenStreamExt};
use syn::parse::Parse;
pub(crate) struct Named{
    /// Attributes tagged on the field.
    pub attrs: Vec<syn::Attribute>,

    /// Visibility of the field.
    pub vis: syn::Visibility,

    /// Name of the field
    pub ident: syn::Ident,

    pub colon_token: syn::Token![:],

    /// Type of the field.
    pub ty: syn::Type,
}
impl Parse for Named {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Named {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Into<syn::Field> for Named {
    fn into(self) -> syn::Field {
        let Self {
            attrs, vis, ident, colon_token, ty,
        } = self;
        syn::Field {
            attrs, vis, ident: Some(ident), colon_token: Some(colon_token), ty,
        }
    }
}

impl ToTokens for Named {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(&self.attrs);
        self.vis.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

pub(crate) struct Unnamed{
    /// Attributes tagged on the field.
    pub attrs: Vec<syn::Attribute>,

    /// Visibility of the field.
    pub vis: syn::Visibility,

    /// Type of the field.
    pub ty: syn::Type,
}
impl Parse for Unnamed {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Unnamed {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Into<syn::Field> for Unnamed {
    fn into(self) -> syn::Field {
        let Self {
            attrs, vis, ty,
        } = self;
        syn::Field {
            attrs, vis, ident: None, colon_token: None, ty,
        }
    }
}

impl ToTokens for Unnamed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(&self.attrs);
        self.vis.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}