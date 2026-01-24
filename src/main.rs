mod geoip;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

const _VERSION_: &'static str = "v1.0.1";

#[get("", "/", "/ip")]
async fn ip() -> impl Responder {
    HttpResponse::Ok().json(&format!("Hello!"))
}

#[get("/ip/info/{ip}")]
async fn ip_info(ip: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(&format!("Hello {ip}!"))
}

#[get("/ip6/info/{ip}")]
async fn ip6_info(ip: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(&format!("Hello {ip}!"))
}

#[get("/version")]
async fn _version() -> impl Responder {
    HttpResponse::Ok().json(&format!("{_VERSION_}"))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("AfxIP starting...");

    HttpServer::new(|| {
        App::new() //
            .service(_version) //
            .service(ip_info) //
            .service(ip6_info) //
    })
        .bind(("0.0.0.0", 11089))?
        .run()
        .await
}
