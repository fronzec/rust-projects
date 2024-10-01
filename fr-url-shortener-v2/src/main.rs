#[macro_use]
extern crate rocket;

use std::fmt::format;

#[get("/")]
fn index() -> &'static str {
    "/"
}

/// This endpoint is used to check the availability and responsiveness of the service. It acts as a health check endpoint.
#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, ping, hello])
}