pub use openapi_derive::derive_openapi as openapi;

pub trait Decoder<S> {
    async fn decode(req: &mut tide::Request<S>) -> tide::Result<Self>;
}

pub struct Params<T, U> {
    path: T,
    query: U,
}

impl<S, T, U> Decoder<S> for Params<T, U> where T: PathDecoder, U: serde::Deserialize {
    async fn decode(req: &mut tide::Request<T>) -> tide::Result<Self> {
        Ok(Self {
            path: <T as PathDecoder>::path_decoder(req),
            query: req.query()?,
        })
    }
}

pub trait PathDecoder<S> {
    fn path_decoder(req: &mut tide::Request<T>) -> tide::Result<Self>;
}