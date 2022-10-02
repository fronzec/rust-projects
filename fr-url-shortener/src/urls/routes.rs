use actix_web::{get, HttpResponse, post, Responder, web};

use super::dto::{ShortenedUrlDto, ShortenUrlDto};

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

/// Configures the Urls routes
pub fn config_url_routes(config: &mut web::ServiceConfig) {
    config.service(shorten_url);
    config.service(redirect_to_target_url);
}