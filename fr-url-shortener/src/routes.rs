use actix_web::{get, HttpResponse, post, Responder, web};
use serde::Deserialize;
use serde::Serialize;

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

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(root);
    config.service(ping);
    config.service(shorten_url);
    config.service(redirect_to_target_url);
}
