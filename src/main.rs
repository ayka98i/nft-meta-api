use actix_web::{guard, http, web, App, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("./local_keys/key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("./local_keys/cert.pem")
        .unwrap();

    HttpServer::new(|| App::new().configure(ping_config))
        .bind_openssl("localhost:8080", builder)?
        .run()
        .await
}

fn ping_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ping")
            .guard(guard::Method(http::Method::GET))
            .route(web::get().to(|| async { HttpResponse::Ok().body("pong") })),
    );
}
