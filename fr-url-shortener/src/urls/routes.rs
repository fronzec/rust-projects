use actix_web::{get, post, web, HttpResponse, Responder};

use crate::db;
use crate::models::Url;

use super::actions;
use super::dto::{ShortenUrlDto, ShortenedUrlDto};

#[post("/shorten-url")]
async fn shorten_url(
    pool: web::Data<db::Pool>,
    payload: web::Json<ShortenUrlDto>,
) -> impl Responder {
    // Run query using Diesel to insert a new database row and return the result.
    let response = web::block(move || {
        let conn = pool.get().expect("couldn't get db connection from pool");
        let new_url = Url {
            id: 10,
            key: "aaaaa".to_string(),
            secret_key: "bbbbb".to_string(),
            target_url: payload.target_url.to_string(),
            is_active: true,
            clicks: 0,
            created_at: Default::default(),
            updated_at: Default::default(),
        };
        actions::insert_new_url(conn, new_url)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })
    .unwrap();

    match response {
        Ok(url) => {
            let shortened = ShortenedUrlDto {
                id: url.id as u64, // Example casting in rust
                target_url: url.target_url.to_owned(),
                url: String::from(url.target_url.to_owned()),
                is_active: url.is_active,
                clicks_count: url.clicks as u32,
                admin_url: String::from(url.target_url.clone()),
            };
            HttpResponse::Ok().json(web::Json(shortened))
        }
        Err(err) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{url_key}")]
async fn redirect_to_target_url(url_key: web::Path<String>) -> impl Responder {
    let target_url = "https://fronzec.io";
    HttpResponse::TemporaryRedirect()
        .insert_header(("Location", target_url))
        .finish()
}

/// Configures the Urls routes
pub fn config_url_routes(config: &mut web::ServiceConfig) {
    config.service(shorten_url);
    config.service(redirect_to_target_url);
}
