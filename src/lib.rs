pub use openapi_derive::derive_openapi as openapi;

pub use mime;

#[::async_trait::async_trait]
pub trait Decoder<'de, S>: Sized {
    fn decode(req: &'de mut tide::Request<S>) -> tide::Result<Self>;
}

pub struct Params<T, U> {
    path: T,
    query: U,
}

impl<'de, S, T, U> Decoder<'de, S> for Params<T, U> where T: PathDecoder<S>, U: serde::Deserialize<'de> {
    fn decode(req: &'de mut tide::Request<S>) -> tide::Result<Self> {
        Ok(Self {
            path: <T as PathDecoder<S>>::path_decoder(req)?,
            query: req.query()?,
        })
    }
}

pub trait PathDecoder<S>: Sized {
    fn path_decoder(req: &mut tide::Request<S>) -> tide::Result<Self>;
}