use crate::handlers;

use actix_web::{middleware, web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_rt::main]
pub async fn start_server(port: u16) -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let app = || {
        App::new()
            .wrap(middleware::NormalizePath)
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::scope("").configure(api_config))
    };

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("ca-key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("ca-cert.pem").unwrap();

    HttpServer::new(app)
        .bind_openssl(format!("{}:{}", "localhost", port), builder)?
        .workers(1)
        .run()
        .await
}

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").configure(handlers::watcher::handler_config));
}
