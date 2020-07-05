/// The Foo API
mod server {
    pub mod components {
        pub mod schemas {

            #[derive(::serde::Serialize, ::serde::Deserialize)]
            /// Whether maybeGetName should return name
            pub struct Req {
                pub some: ::std::option::Option<bool>,
            }

            #[derive(::serde::Serialize, ::serde::Deserialize)]
            /// Response for maybe get name
            pub struct Resp {
                pub name: ::std::string::String,
            }
            
            #[derive(::serde::Serialize, ::serde::Deserialize)]
            pub struct FoobarBaz {
                pub qux: ::std::option::Option<::std::string::String>,
            }
        
            #[derive(::serde::Serialize, ::serde::Deserialize)]
            pub struct Foobar {
                pub baz: ::std::option::Option<FoobarBaz>,
            }
            
            #[derive(::serde::Serialize, ::serde::Deserialize)]
            enum EnumTest {
                #[serde(rename = "foo")]
                Foo,
                #[serde(rename = "bar")]
                Bar,
                #[serde(rename = "baz")]
                Baz,
                #[serde(rename = "LOL")]
                Lol,
                #[serde(rename = "Multiple words")]
                MultipleWords,
            }
        }
    }

    pub mod paths {

        /// /bar
        pub mod bar {
            /// maybeGetName
            /// POST: /bar
            pub mod post {
                /// Responses for maybeGetName
                pub mod responses {
                    /// Content for maybeGetName =>  404 (Not Found)
                    #[derive(::serde::Serialize)]
                    pub struct ContentNotFound {
                        pub nothing: bool,
                    }

                    /// Headers for maybeGetName =>  404 (Not Found)
                    pub struct HeadersNotFound {
                        /// X-Example-Header
                        pub x_example_header: Option<String>,
                    }

                    /// Response for maybeGetName
                    pub enum Response {
                        /// 200 (Success) Response for maybeGetName - JSON Encoded
                        SuccessJSON(super::super::super::super::components::schemas::Resp),
                        /// 200 (Success) Response for maybeGetName - MSGPACK Encoded
                        SuccessMSGPACK(super::super::super::super::components::schemas::Resp),
                        /// 404 (Not Found) Response for maybeGetName
                        NotFound {
                            headers: HeadersNotFound,
                            content: ContentNotFound,
                        },
                        /// 400 (Bad Request) Response for maybeGetName - YAML Encoded
                        BadRequest,
                    }

                    impl Response {
                        pub fn encode(self) -> ::tide::Result {
                            match self {
                                Response::SuccessJSON(content) => {
                                    let mut response = ::tide::Response::new(::tide::StatusCode::Ok);
                                    response.set_body(::serde_json::to_vec(&content)?);
                                    Ok(response.set_mime(::openapi_tide::mime::APPLICATION_JSON))
                                }
                                Response::SuccessMSGPACK(content) => {
                                    let mut response = ::tide::Response::new(::tide::StatusCode::Ok);
                                    response.set_body(::rmp_serde::to_vec(&content)?);
                                    Ok(response.set_mime(::openapi_tide::mime::APPLICATION_MSGPACK))
                                }
                                Response::NotFound{headers, content} => {
                                    let mut response = ::tide::Response::new(::tide::StatusCode::NotFound);
                                    response.set_body(::serde_json::to_vec(&content)?);
                                    let response = match headers.x_example_header {
                                        Some(x_example_header) => response.set_header("X-Example-Header", x_example_header),
                                        _ => response,
                                    };
                                    Ok(response.set_mime(::openapi_tide::mime::APPLICATION_JSON))
                                }
                                Response::BadRequest => {
                                    let response = ::tide::Response::new(::tide::StatusCode::NotFound);
                                    Ok(response)
                                }
                            }
                        }
                    }
                }

                /// Request for maybeGetName
                pub mod request {
                    pub type Body = super::super::super::super::components::schemas::Req;

                    pub async fn decode<S>(req: &mut ::tide::Request<S>) -> tide::Result<(Body,)> {
                        let content_type = req.as_ref().content_type().ok_or(::tide::Error::from_str(::tide::StatusCode::BadRequest, "expected Content-Type header"))?;
                        let body = match content_type.essence() {
                            "application/json" => {
                                Ok(::serde_json::from_slice(&req.body_bytes().await?)?)
                            },
                            _ => Err(::tide::Error::from_str(::tide::StatusCode::BadRequest, "unexpected Content-Type value"))
                        }?;

                        Ok((body,))
                    }
                }
                
                #[::async_trait::async_trait]
                pub trait Spec: Sized + Send + Sync + 'static {
                    async fn maybe_get_name(&self, body: Option<request::Body>, req: &::tide::Request<Self>) -> ::tide::Result<responses::Response>;
                }

                pub async fn endpoint<'a, S>(mut req: ::tide::Request<S>) -> ::tide::Result where S: Spec {
                    let (body,) = request::decode(&mut req).await?;
                    Spec::maybe_get_name(req.state(), Some(body), &req).await?.encode()
                }
            }

            pub fn add_route<S>(route: &mut ::tide::Route<S>) where S: Spec {
                route
                    .post(post::endpoint)
                    ;
            }

            pub trait Spec: post::Spec {}
        }

        pub trait Spec: bar::Spec {}
    }

    pub trait Spec: paths::Spec {
        fn into_server(self) -> tide::Server<Self> {
            let mut app = tide::with_state(self);

            paths::bar::add_route(&mut app.at("/bar"));
            
            app
        }
    }

    pub const VERSION: &str = "0.0.1";
    pub const TITLE: &str = "Foo Spec";
}

struct Server {
    name: String,
}

use server::components::schemas::*;

#[::async_trait::async_trait]
impl server::paths::bar::post::Spec for Server {
    async fn maybe_get_name(&self, body: Option<server::paths::bar::post::request::Body>, req: &::tide::Request<Self>) -> ::tide::Result<server::paths::bar::post::responses::Response> {
        use server::paths::bar::post::responses::*;

        Ok(match body {
            None => Response::BadRequest,
            Some(req) => if req.some.unwrap_or(false) {
                Response::SuccessJSON(Resp {
                    name: self.name.clone()
                })
            } else {
                Response::NotFound {
                    headers: HeadersNotFound {
                        x_example_header: Some("example header".to_string()),
                    },
                    content: ContentNotFound {
                        nothing: true,
                    },
                }
            },
        })
    }
}

impl server::paths::bar::Spec for Server{}
impl server::paths::Spec for Server{}
impl server::Spec for Server{}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Launching server {} with version {}", server::TITLE, server::VERSION);

    let server = server::Spec::into_server(Server{name: "foo".to_string()});
    
    server.listen("127.0.0.1:3001").await?;

    Ok(())
}