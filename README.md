# openapi-tide
OpenAPI procedural macro for rust built upon https://github.com/http-rs/tide

## Example
main.rs:
```rust
use openapi_tide::openapi;
use async_trait::async_trait;
use tide::{Request, Result, StatusCode};

#[openapi("/path/to/specfile")]
mod server {}

struct Server(String);

#[async_trait]
impl server.Spec for Server {
    async fn maybeGetName(&self, body: server.Req, _req: &Request<Self>) -> Result<(server.Resp, StatusCode)> {
        if body.some {
            Ok((server.Resp{name: Some(self.0.clone())}, StatusCode::Ok))
        } else {
            Ok((server.Resp{name: None}, StatusCode::NotFound))
        }
    }
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let s = Server{name: "foo".to_string()};
    let s = s.into_server();
    s.listen("127.0.0.1:3001").await?;
    Ok(())
}
```

specfile.yaml:
```yaml
openapi: 3.0.0

info: ...

paths:
    /bar:
        post:
            description: get name of server if some is set to true
            operationId: maybeGetName
            requestBody:
                content:
                    application/json:
                        schema:
                            $ref: "#/components/req"
                required: true
            responses:
                200:
                    description: Success Response
                    content:
                        application/json:
                            $ref: '#/components/resp'
                404:
                    description: Not Found.
                    content:
                        application/json:
                            $ref: '#/components/resp'
                500:
                    description: Internal server error
                    content:
                        application/json:
                            schema: {}

components:
    schemas:
        req:
            type: object
            example:
                some: true
            properties:
                some:
                    type: bool
            required:
                - some
        resp:
            type: object
            example:
                name: "Name"
            properties:
                name:
                    type: string
```

The server module would expand to something like 
```rust
mod server {
    #[derive(serde::Deserialize)]
    struct Req {
        some: bool,
    }

    #[derive(serde::Serialize)]
    struct Resp {
        name: Option<String>,
    }

    #[::async_trait::async_trait]
    trait Spec: Sized + Send + Sync + 'static {

        /// get name of server if some is set to true
        /// POST: /bar
        async fn maybeGetName(&self, body: Req, req: &tide::Request<Self>) -> tide::Result<(Resp, tide::StatusCode)>;

        fn into_server(self) -> tide::Server<Self> {
            let mut app = tide::with_state(self);

            app.at("/bar").post(
                |mut req: tide::Request<Self>| async move {
                    let body: Req = req.body_json().await?;
                    let (resp, status) = req.state().maybeGetName(body, &req).await?;
                    Ok(tide::Response::new(status).body_json(&resp)?)
                }
            );
            
            app
        }
    }
}
```