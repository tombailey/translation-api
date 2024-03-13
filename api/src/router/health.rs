use crate::dependency::translation::Translator;
use crate::AppState;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse};
use log::warn;
use serde::Serialize;
use serde_json::json;
use translation::HealthCheck;

#[derive(Serialize)]
struct Healthy {
    status: String,
}

#[get("/health")]
pub async fn get_health(app_state: web::Data<AppState<Translator>>) -> HttpResponse {
    let is_healthy = app_state.translator.is_healthy().await.unwrap_or(true);
    if is_healthy {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(json!({ "status": "pass" }))
    } else {
        warn!("Translator is unhealthy");
        HttpResponse::ServiceUnavailable().finish()
    }
}
