#![feature(trait_alias)]

mod dependency;
mod router;

use crate::dependency::translation::get_first_configured_translator;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env::require_env_var;
use router::health::get_health;
use router::translate::translate;
use std::sync::Arc;
use translation::TranslationProvider;

pub struct AppState<TP: TranslationProvider> {
    translator: Arc<TP>,
}

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let port = require_env_var("PORT")
        .expect("Missing port.")
        .parse::<u16>()
        .expect("Invalid port.");

    let translator =
        get_first_configured_translator().expect("No properly configured translation provider.");

    let app_data = web::Data::new(AppState {
        translator: Arc::new(translator),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .service(translate)
            .service(get_health)
    })
    .bind(("0.0.0.0", port))
    .expect("Failed to start server")
    .run()
    .await
    .expect("Failed to start server");
}
