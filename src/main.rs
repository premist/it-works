#[macro_use] extern crate nickel;

use nickel::status::StatusCode;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware! {
      (StatusCode::Ok, "<h1>It works!</h1>")
    });

    server.get("/healthz", middleware! {
      (StatusCode::Ok, "")
    });

    server.listen("127.0.0.1:8080").unwrap();
}
