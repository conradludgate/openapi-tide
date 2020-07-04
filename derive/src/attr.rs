use syn::{LitStr, parse::{ParseStream, Parse}, Result};

pub(crate) struct Attributes {
    pub(crate) spec_file: LitStr,
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Attributes{
            spec_file: input.parse()?,
        })
    }
}