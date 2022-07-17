use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    response: String,
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("URL shortener")
}

#[get("/ping")]
async fn ping() -> impl Responder {
    let obj = MyObj { response: "pong".parse().unwrap() };
    HttpResponse::Ok().json(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(ping)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
