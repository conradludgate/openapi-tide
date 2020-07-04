use openapi_tide::openapi;

#[openapi("examples/MVP/spec.yaml")]
mod server {}

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