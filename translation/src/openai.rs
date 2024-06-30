use crate::{
    HealthCheck, Language, Translation, TranslationError, TranslationInput, TranslationOutput,
    TranslationProvider,
};
use async_trait::async_trait;
use futures::future::join_all;
use itertools::Itertools;
use openai::client::OpenAIClient;
use openai::error::OpenAIError;

pub struct OpenAITranslationProvider {
    open_ai_client: OpenAIClient,
}

impl OpenAITranslationProvider {
    pub fn new(open_ai_client: OpenAIClient) -> Self {
        OpenAITranslationProvider { open_ai_client }
    }
}

#[async_trait]
impl HealthCheck for OpenAITranslationProvider {
    async fn is_healthy(&self) -> Option<bool> {
        Some(self.open_ai_client.get_models().await.is_ok())
    }
}

impl Translation for OpenAITranslationProvider {
    async fn translate(
        &self,
        inputs: Vec<TranslationInput>,
    ) -> Result<Vec<TranslationOutput>, TranslationError> {
        Ok(join_all(
            inputs
                .into_iter()
                .map(|input| {
                    let from_source = format!(
                        " from {}",
                        input.source_language.map(|source| Language::to_string(&source)).unwrap_or("".to_owned())
                    );
                    let system_prompt = format!(
                        "Please translate the user's text{from_source} to {}, only respond with the translation",
                        input.target_language.to_string()
                    );
                    self.open_ai_client.respond_to(system_prompt, input.text, None)
                })
                .collect_vec(),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<String>, OpenAIError>>()?
        .into_iter()
        .map(|response| TranslationOutput {
            text: response,
            source_language: None,
        })
        .collect())
    }
}

impl TranslationProvider for OpenAITranslationProvider {}
