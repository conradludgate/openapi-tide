#[derive(serde::Deserialize)]
struct Req {
    some: bool,
}

#[derive(serde::Serialize)]
struct Res {
    name: Option<String>,
}

enum BarResponses {
    SuccessResponse(Res),
    NotFound(Res),
    BadRequest,
}

impl Into<tide::Response> for BarResponses {
    fn into(self) -> tide::Response {
        match self {
            BarResponses::SuccessResponse(r) => {
                tide::Response::new(tide::StatusCode::Ok)
                    .body_json(&r)
                    .unwrap() 
            },
            BarResponses::NotFound(r) => {
                tide::Response::new(tide::StatusCode::NotFound)
                    .body_json(&r)
                    .unwrap()
            },
            BarResponses::BadRequest => {
                tide::Response::new(tide::StatusCode::BadRequest)
            }
        }
    }
}

#[::async_trait::async_trait]
trait Foo: Sized + Send + Sync + 'static {

    /// /bar
    async fn bar(&self, body: Option<Req>, req: &tide::Request<Self>) -> BarResponses;

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
    async fn bar(&self, body: Option<Req>, _req: &tide::Request<Self>) -> BarResponses {
        match body {
            None => BarResponses::BadRequest,
            Some(Req{some: true}) => BarResponses::SuccessResponse(Res{name: Some(self.name.clone())}),
            Some(Req{some: false}) => BarResponses::NotFound(Res{name: None}),
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