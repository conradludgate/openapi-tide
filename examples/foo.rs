#[derive(serde::Deserialize)]
struct Req {
    some: bool,
}

#[derive(serde::Serialize)]
struct Res {
    name: Option<String>,
}

enum BarResponse {
    SuccessResponse(Res),
    NotFound(Res),
    BadRequest,
}

impl Into<tide::Response> for BarResponse {
    fn into(self) -> tide::Response {
        match self {
            BarResponse::SuccessResponse(r) => {
                tide::Response::new(tide::StatusCode::Ok)
                    .body_json(&r)
                    .unwrap() 
            },
            BarResponse::NotFound(r) => {
                tide::Response::new(tide::StatusCode::NotFound)
                    .body_json(&r)
                    .unwrap()
            },
            BarResponse::BadRequest => {
                tide::Response::new(tide::StatusCode::BadRequest)
            }
        }
    }
}

#[::async_trait::async_trait]
trait Foo: Sized + Send + Sync + 'static {

    /// /bar
    async fn bar(&self, body: Option<Req>, req: &tide::Request<Self>) -> BarResponse;

    fn into_server(self) -> tide::Server<Self> {
        let mut app = tide::with_state(self);

        app.at("/bar").post(
            |mut req: tide::Request<Self>| async move {
                let body: Option<Req> = req.body_json().await.ok();
                Ok(req.state().bar(body, &req).await)
            }
        );
        
        app
    }
}

struct Baz {
    name: String,
}

#[::async_trait::async_trait]
impl Foo for Baz {
    async fn bar(&self, body: Option<Req>, _req: &tide::Request<Self>) -> BarResponse {
        match body {
            None => BarResponse::BadRequest,
            Some(Req{some: true}) => BarResponse::SuccessResponse(Res{name: Some(self.name.clone())}),
            Some(Req{some: false}) => BarResponse::NotFound(Res{name: None}),
        }
    }
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let baz = Baz{name: "foo".to_string()};
    let server = baz.into_server();
    server.listen("127.0.0.1:3001").await?;
    Ok(())
}