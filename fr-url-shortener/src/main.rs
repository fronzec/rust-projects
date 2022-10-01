#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

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
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Env vars
    let res = config::configs::log_envs();
    match res {
        Ok(s) => s,
        Err(error) => {
            println!("failed to load config vars for app, detail= {}{}", error.kind().to_string(), error.to_string());
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
