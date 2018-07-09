extern crate actix_web;

use actix_web::{
    http,
    http::StatusCode,
    server,
    App,
    Responder,
    HttpRequest,
    HttpResponse,
    error,
    middleware::{
        Middleware,
        Started,
    },
};

struct cURLFilter;
impl<S> Middleware<S> for cURLFilter {
    fn start(&self, req: &mut HttpRequest<S>) -> error::Result<Started> {
        match req.headers_mut().get("user-agent") {
            Some(user_agent) if user_agent.to_str().unwrap().contains("curl") => Ok(Started::Done),
            _ => Ok(Started::Response(HttpResponse::new(StatusCode::BAD_REQUEST))),
        }
    }
}

fn hello_name(req: HttpRequest) -> impl Responder {
    println!("hello_name: {:?}", req);
    "ok"
}

fn main() {
    server::new(|| {
        App::new()
            .middleware(cURLFilter)
            .route("/hello/{name}", http::Method::GET, hello_name)
    }).bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
