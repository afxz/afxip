mod geoip;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, routes, web};
use std::net::IpAddr;

const _VERSION_: &'static str = "v1.0.1";

#[get("/ip")]
async fn ip(_req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let _ip = _req
        .headers()
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|xff| {
            xff.split(',')
                .map(|s| s.trim())
                .find(|s| !s.eq_ignore_ascii_case("unknown"))
                .and_then(|s| s.parse::<IpAddr>().ok())
        })
        .unwrap_or_else(|| {
            _req.peer_addr()
                .map(|addr| addr.ip())
                .unwrap_or_else(|| "unknown".parse::<IpAddr>().unwrap())
        });

    Ok(HttpResponse::Ok().json(format!("IP: {}", _ip)))
}

#[get("/ip/info/{ip}")]
async fn ip_info(_ip: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(&format!("Hello {_ip}!"))
}

#[get("/ip6/info/{ip}")]
async fn ip6_info(_ip: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(&format!("Hello {_ip}!"))
}

#[routes]
#[get("/")]
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
            .service(ip) //
            .service(ip_info) //
            .service(ip6_info) //
    })
    .bind(("0.0.0.0", 11089))?
    .run()
    .await
}
