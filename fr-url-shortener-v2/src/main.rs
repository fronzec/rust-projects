#[macro_use]
extern crate rocket;

use std::fmt::format;
use std::path::{Path, PathBuf};
use rocket::fs::{NamedFile, FileServer};

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

/// More complex GET endpoint
#[get("/hello/<name>/<age>/<cool>")]
fn hello_world(name: String, age: i32, cool: bool) -> String {
    if cool {
        format!("Hello, {} year old named {}!", age, name)
    } else {
        format!("{} you aren't very cool yet", name)
    }
}

/// Serving static files with Multiple Segments, PathBuf is secure against path traversal attacks.
#[get("/static/<file..>")]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

/// Using launch is the recommended way to run an app using rocket
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, ping, hello, hello_world, static_files])
        // other easier way to serve files from `/static` at path `/public`
        .mount("/public", FileServer::from("static"))
}