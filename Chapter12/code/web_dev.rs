// this file does not compile
// it shows only "hello-world" example snippets for the most popular web frameworks. 
// iron framework:
extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("localhost:3000").unwrap();
}

// nickel framework:
#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    server.get("**", middleware!("Hello World"));
    server.listen("127.0.0.1:6767");
}

// rocket framework:
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

// gotham framework:
extern crate futures;
extern crate hyper;
extern crate gotham;
extern crate mime;

use hyper::server::Http;
use hyper::{Request, Response, StatusCode};

use gotham::http::response::create_response;
use gotham::state::State;
use gotham::handler::NewHandlerService;

pub fn say_hello(state: State, _req: Request) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("Hello World!").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );

    (state, res)
}

pub fn main() {
    let addr = "127.0.0.1:7878".parse().unwrap();

    let server = Http::new()
        .bind(&addr, NewHandlerService::new(|| Ok(say_hello)))
        .unwrap();

    println!(
        "Listening on http://{}",
        server.local_addr().unwrap()
    );

    server.run().unwrap();
}