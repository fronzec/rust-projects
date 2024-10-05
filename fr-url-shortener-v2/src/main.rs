#[macro_use]
extern crate rocket;

use log;
use std::fmt::format;
use std::path::{Path, PathBuf};
use rocket::fs::{NamedFile, FileServer};
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
struct ShortenRequest<'r> {
    url: &'r str
}
#[post("/", format="json", data="<payload>")]
fn short_url(payload: Json<ShortenRequest<'_>>) -> Value {
    log::info!("shorten: {:?}", payload.into_inner());
    json!({ "status": "ok", "url": "short_url" })
}

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
        .mount("/short-url", routes![short_url])
        // other easier way to serve files from `/static` at path `/public`
        .mount("/public", FileServer::from("static"))
}