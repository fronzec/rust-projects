use actix_web::{get, HttpResponse, Responder, web};
use serde::Serialize;

#[derive(Serialize)]
pub struct MyObj {
    response: String,
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

pub fn basic_routes(config: &mut web::ServiceConfig) {
    config.service(root);
    config.service(ping);
}
