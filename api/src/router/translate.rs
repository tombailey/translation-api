use crate::dependency::translation::Translator;
use crate::AppState;
use actix_web::{post, web, HttpResponse, ResponseError};
use log::error;
use thiserror::Error;
use translation::{Translation, TranslationError, TranslationInput};

#[derive(Error, Debug)]
pub enum TranslateRouteError {
    #[error("TranslationError: {0}")]
    TranslationError(#[from] TranslationError),
}

impl ResponseError for TranslateRouteError {
    fn error_response(&self) -> HttpResponse {
        error!("{}", self);
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/translate")]
pub async fn translate(
    (translation_input, app_state): (
        web::Json<Vec<TranslationInput>>,
        web::Data<AppState<Translator>>,
    ),
) -> Result<HttpResponse, TranslateRouteError> {
    let output = app_state.translator.translate(translation_input.0).await?;
    Ok(HttpResponse::Ok().json(output))
}
