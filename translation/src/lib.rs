#![feature(trait_alias)]

pub mod claude;
pub mod deepl;
pub mod openai;

use ::claude::error::ClaudeError;
use ::deepl::error::DeepLError;
use ::openai::error::OpenAIError;
use async_trait::async_trait;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_with::DeserializeFromStr;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, DeserializeFromStr, Serialize)]
pub struct Language(isolang::Language);

#[derive(Debug, Display)]
pub struct LanguageError;

impl FromStr for Language {
    type Err = LanguageError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        isolang::Language::from_str(value)
            .or(isolang::Language::from_name(value).ok_or(LanguageError))
            .map(Language)
            .map_err(|_| LanguageError)
    }
}

impl Language {
    fn to_string(&self) -> String {
        self.0
            .to_639_1()
            .map(str::to_owned)
            .unwrap_or(self.0.to_string())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TranslationInput {
    #[serde(rename = "input")]
    text: String,
    #[serde(rename = "source")]
    source_language: Option<Language>,
    #[serde(rename = "target")]
    target_language: Language,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TranslationOutput {
    #[serde(rename = "output")]
    text: String,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    source_language: Option<Language>,
}

#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("ClaudeError {0}")]
    ClaudeError(#[from] ClaudeError),
    #[error("DeepLError {0}")]
    DeepLError(#[from] DeepLError),
    #[error("OpenAIError {0}")]
    OpenAIError(#[from] OpenAIError),
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
