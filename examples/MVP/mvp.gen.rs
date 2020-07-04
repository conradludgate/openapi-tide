/// The Foo API
mod server {
    pub mod components {
        pub mod schemas {

            /// Whether maybeGetName should return name
            pub struct Req {
                pub some: ::std::option::Option<bool>,
            }

            /// Response for maybe get name
            pub struct Resp {
                pub name: ::std::string::String,
            }
            
            pub struct FoobarBaz {
                pub qux: ::std::option::Option<::std::string::String>,
            }
        
            pub struct Foobar {
                pub baz: ::std::option::Option<FoobarBaz>,
            }
            
            enum EnumTest {
                Foo = "foo",
                Bar = "bar",
                Baz = "baz",
                Lol = "LOL",
                MultipleWords = "Multiple words",
            }
        }
    }

    pub mod paths {

        /// /bar
        pub mod Bar {
            /// maybeGetName
            /// POST: /bar
            pub mod Post {
                /// Responses for maybeGetName
                pub mod Responses {
                    /// Content for maybeGetName =>  404 (Not Found)
                    pub struct ContentNotFound {
                        pub nothing: bool,
                    }

                    /// Headers for maybeGetName =>  404 (Not Found)
                    pub struct HeadersNotFound {
                        /// X-Example-Header
                        x_example_header: Option<String>,
                    }

                    /// Response for maybeGetName
                    pub enum Response {
                        /// 200 (Success) Response for maybeGetName - JSON Encoded
                        SuccessJSON(super::super::super::super::components::schemas::Resp),
                        /// 200 (Success) Response for maybeGetName - YAML Encoded
                        SuccessYAML(super::super::super::super::components::schemas::Resp),
                        /// 404 (Not Found) Response for maybeGetName
                        NotFound {
                            headers: HeadersNotFound,
                            content: ContentNotFound,
                        },
                        /// 400 (Bad Request) Response for maybeGetName - YAML Encoded
                        BadRequest,
                    }

                    impl Response {
                        fn encode(self) -> ::tide::Result {
                            match self {
                                Response::SuccessJSON(content) => {
                                    let response = ::tide::Response::new(200);
                                    response.set_body(::serde_json::to_vec(content)?);
                                    Ok(response.set_mime(::openapi_tide::mime::APPLICATION_JSON))
                                }
                                Response::SuccessYAML(content) => {
                                    let response = ::tide::Response::new(200);
                                    response.set_body(::serde_yaml::to_vec(content)?);
                                    Ok(response.set_mime(::openapi_tide::mime::APPLICATION_YAML))
                                }
                                Response::NotFound{headers, content} => {
                                    let response = ::tide::Response::new(404);
                                    response.set_body(::serde_json::to_vec(content)?);
                                    let response = match headers.x_example_header {
                                        Some(x_example_header) => response.set_header("X-Example-Header", x_example_header),
                                        _ => response,
                                    };
                                    Ok(response.set_mime(::openapi_tide::mime::APPLICATION_JSON))
                                }
                                Response::BadRequest => {
                                    let response = ::tide::Response::new(400);
                                    Ok(response)
                                }
                            }
                        }
                    }
                }

                /// Request for maybeGetName
                pub mod Request {
                    pub type Body = super::super::super::components::schemas::Req;

                    pub fn decode<S>(req: &mut ::tide::Request<S>) -> tide::Result<(Body,)> {
                        let content_type = req.content_type().ok_or(::tide::Error::new(400, "expected Content-Type header"))?;
                        let body = match content_type.essence() {
                            "application/json" => {
                                ::serde_json::from_slice(req.body_bytes()?)
                            },
                            _ => Err(::tide::Error::new(400, "unexpected Content-Type value"))
                        }?;
                        Ok((body,))
                    }
                }
                

                pub trait Spec: Sized + Send + Sync + 'static {
                    async fn maybeGetName(&self, body: Option<Request::Body>, req: &::tide::Request<Self>) -> ::tide::Result<Responses>;
                }

                pub async fn endpoint<'a, S>(mut req: ::tide::Request<S>) -> ::tide::Result where S: Spec {
                    let (body,) = Request::decode(&mut req)?;
                    super::super::Spec::maybeGetName(body, req.state(), &req).await?.encode()
                }
            }

            pub fn add_route<S>(route: &mut ::tide::Route<S>) where S: Spec {
                route
                    .post(Post::endpoint)
            }

            pub trait Spec: Post::Spec {}
        }

        pub trait Spec: Bar::Spec {}
    }

    trait Spec: paths::Spec {
        fn into_server(self) -> tide::Server<Self> {
            let mut app = tide::with_state(self);

            paths::Bar::add_route(&mut app.at("/bar"));
            
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
impl server::Spec for Server {
    async fn maybeGetName(&self, body: Option<server::paths::Bar::Post::Request::Body>, req: &::tide::Request<Self>) -> ::tide::Result<server::paths::Bar::Post::Responses::Response> {
        use server::paths::Bar::Post::Request::*;
        use server::paths::Bar::Post::Responses::*;

        match body {
            None => Response::BadRequest,
            Some(req) => if req.some {
                Response::SuccessResponse(Resp {
                    name: Some(self.name.clone())
                })
            } else {
                Response::NotFound {
                    headers: HeadersNotFound {
                        x_example_header: Some("example header".to_string()),
                    },
                    content: NotFound {
                        name: None,
                    },
                }
            },
        }
    }
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Launching server {} with version {}", server::TITLE, server::VERSION);

    let server = Server{name: "foo".to_string()}.into_server();
    
    server.listen("127.0.0.1:3001").await?;

    Ok(())
}