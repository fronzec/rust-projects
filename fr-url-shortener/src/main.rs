#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::process::exit;

use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::Url;

mod config;
mod db;
mod models;
mod routes;
mod schema;
mod urls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configs
    let res = config::configs::load_configs();
    match res {
        Ok(s) => s,
        Err(_error) => {
            eprintln!("---> app: failed on configuration step, exiting");
            exit(1)
        }
    }

    println!("starting db init");
    db::init();
    println!("ok db init");

    // test load urls using a connection from the pool

    println!("db connection initializing");
    let conn = db::connection()?;
println!("connection pool ok");
    test_load_urls(&conn);
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(routes::basic_routes)
            .configure(urls::routes::config_url_routes)
            .app_data(web::Data::new(db::get_pool_clone()))
    });

    server.bind(("127.0.0.1", 8080))?.run().await
}

pub fn test_load_urls(connection: &PgConnection) -> () {
    use crate::schema::urls::dsl::*;

    let results = urls
        .filter(is_active.eq(true))
        .limit(5)
        .load::<Url>(connection)
        .expect("----> app: error loading urls step, exiting");

    println!("Displaying {} urls", results.len());
    for url in results {
        println!("\n----------");
        println!("{}", url.id);
        println!("{}", url.target_url);
    }
}
