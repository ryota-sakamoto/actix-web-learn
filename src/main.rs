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
    pred,
    middleware::{
        Middleware,
        Started,
    },
    Path,
};
use std::{
    time::Duration,
    thread,
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

fn ping(_: HttpRequest) -> impl Responder {
    "pong"
}

fn hello_name(req: HttpRequest) -> impl Responder {
    println!("hello_name: {:?}", req);
    "ok"
}

fn return_binary(_: HttpRequest) -> impl Responder {
    "binary".as_bytes()
}

fn created(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::CREATED)
}

fn sleep(p: Path<(u64)>) -> impl Responder {
    thread::sleep(Duration::from_millis(1000 * p.as_ref()));
    format!("sleep: {} secs", p.as_ref())
}

fn main() {
    server::new(|| {
        App::new()
            .middleware(cURLFilter)
            .resource("/ping", |r| {
                r.route()
                    .filter(pred::Header("Content-Type", "text"))
                    .f(ping)
            })
            .route("/hello/{name}", http::Method::GET, hello_name)
            .route("/binary", http::Method::GET, return_binary)
            .route("/created", http::Method::POST, created)
            .route("/sleep/{time}", http::Method::GET, sleep)
    }).bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
