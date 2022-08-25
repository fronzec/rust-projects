#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

use std::env;
use std::io::{Error, ErrorKind};
use std::process::exit;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::models::Url;

mod models;
mod schema;

#[derive(Serialize)]
struct MyObj {
    response: String,
}

#[derive(Deserialize)]
struct ShortenUrlDto {
    target_url: String,
}

#[derive(Serialize)]
struct ShortenedUrlDto {
    id: u64,
    target_url: String,
    url: String,
    is_active: bool,
    clicks_count: u32,
    admin_url: String,
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("URL shortener")
}

#[get("/ping")]
async fn ping() -> impl Responder {
    let obj = MyObj {
        response: "pong".to_string(),
    };
    HttpResponse::Ok().json(web::Json(obj))
}

#[post("/shorten-url")]
async fn shorten_url(payload: web::Json<ShortenUrlDto>) -> impl Responder {
    let response = ShortenedUrlDto {
        id: 0,
        target_url: payload.target_url.to_string(),
        url: "aaaaa".to_string(),
        is_active: true,
        clicks_count: 0,
        admin_url: "aaaaa".to_string(),
    };
    HttpResponse::Ok().json(web::Json(response))
}

#[get("/{url_key}")]
async fn redirect_to_target_url(url_key: web::Path<String>) -> impl Responder {
    let target_url = "https://fronzec.io";
    HttpResponse::TemporaryRedirect()
        .insert_header(("Location", target_url))
        .finish()
}

fn log_envs() -> Result<(), Error> {
    let user = env::var("APP_DB_USER");
    let password = env::var("APP_DB_PASSWORD");
    let db = env::var("APP_DB_NAME");

    if user.is_err() || password.is_err() || db.is_err() {
        println!("---->(env) one or more required envvar(s) not found, exiting");
        return Err(Error::new(ErrorKind::Other, "some env not found!!!"));
    }

    if user.is_ok() {
        println!("---->(env) user <{}>", user.unwrap());
    }
    if password.is_ok() {
        println!("---->(env) password <***>")
    }
    if db.is_ok() {
        println!("---->(env) db <{}>", db.unwrap())
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Env vars
    let res = log_envs();
    match res {
        Ok(s) => s,
        Err(error) => {
            println!("failed to load env vars for app, detail= {}{}", error.kind().to_string(), error.to_string());
            exit(1)
        }
    }

    let connection = establish_connection();
    load_urls(&connection);

    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(ping)
            .service(shorten_url)
            .service(redirect_to_target_url)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("APP_DATABASE_URL")
        .expect("APP_DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn load_urls(connection: &PgConnection) -> () {
    use crate::schema::urls::dsl::*;

    let results = urls.filter(is_active.eq(true))
        .limit(5)
        .load::<Url>(connection)
        .expect("Error loading urls");

    println!("Displaying {} urls", results.len());
    for url in results {
        println!("\n----------");
        println!("{}", url.id);
        println!("{}", url.target_url);
    }
}