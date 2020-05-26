#[derive(serde::Deserialize)]
struct Req {
    some: bool,
}

#[derive(serde::Serialize)]
struct Res {
    name: Option<String>,
}

#[::async_trait::async_trait]
trait Foo: Sized + Send + Sync + 'static {

    /// /bar
    async fn bar(&self, body: Req, req: &tide::Request<Self>) -> tide::Result<(Res, tide::StatusCode)>;

    fn into_server(self) -> tide::Server<Self> {
        let mut app = tide::with_state(self);

        app.at("/bar").post(
            |mut req: tide::Request<Self>| async move {
                let body: Req = req.body_json().await?;
                let (resp, status) = req.state().bar(body, &req).await?;
                Ok(tide::Response::new(status).body_json(&resp)?)
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
    async fn bar(&self, body: Req, _req: &tide::Request<Self>) -> tide::Result<(Res, tide::StatusCode)> {
        if body.some {
            Ok((Res{name: Some(self.name.clone())}, tide::StatusCode::Ok))
        } else {
            Ok((Res{name: None}, tide::StatusCode::NotFound))
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