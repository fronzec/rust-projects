#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

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
mod db;
mod routes;


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

    db::init();

    let conn = db::connection()?;
    load_urls(&conn);

    let mut server = HttpServer::new(|| App::new().configure(routes::init_routes));
    server.bind(("127.0.0.1", 8080))?
        .run()
        .await
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
