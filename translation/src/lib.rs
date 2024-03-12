#![feature(trait_alias)]

pub mod claude;
pub mod deepl;

use ::claude::error::ClaudeError;
use ::deepl::error::DeepLError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TranslationInput {
    #[serde(rename = "input")]
    text: String,
    #[serde(rename = "source")]
    source_language: Option<String>,
    #[serde(rename = "target")]
    target_language: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TranslationOutput {
    #[serde(rename = "output")]
    text: String,
    #[serde(rename = "source")]
    source_language: Option<String>,
}

#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("ClaudeError {0}")]
    ClaudeError(#[from] ClaudeError),
    #[error("DeepLError {0}")]
    DeepLError(#[from] DeepLError),
}

pub trait Translation {
    async fn translate(
        &self,
        inputs: Vec<TranslationInput>,
    ) -> Result<Vec<TranslationOutput>, TranslationError>;
}

#[async_trait]
pub trait HealthCheck {
    async fn is_healthy(&self) -> Option<bool>;
}

pub trait TranslationProvider: Translation + HealthCheck {}
