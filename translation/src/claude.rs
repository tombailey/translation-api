use crate::{
    HealthCheck, Language, Translation, TranslationError, TranslationInput, TranslationOutput,
    TranslationProvider,
};
use async_trait::async_trait;
use claude::client::ClaudeClient;
use claude::error::ClaudeError;
use futures::future::join_all;
use itertools::Itertools;

pub struct ClaudeTranslationProvider {
    claude_client: ClaudeClient,
}

impl ClaudeTranslationProvider {
    pub fn new(claude_client: ClaudeClient) -> Self {
        ClaudeTranslationProvider { claude_client }
    }
}

#[async_trait]
impl HealthCheck for ClaudeTranslationProvider {
    async fn is_healthy(&self) -> Option<bool> {
        // wait for anthropic to provide a health check or status page
        None
    }
}

impl Translation for ClaudeTranslationProvider {
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
                    let prompt = format!(
                        "Please translate the following text{from_source} to {}, only respond with the translation:\n{}",
                        input.target_language.to_string(), input.text
                    );
                    self.claude_client.respond_to(prompt, None)
                })
                .collect_vec(),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<String>, ClaudeError>>()?
        .into_iter()
        .map(|response| TranslationOutput {
            text: response,
            source_language: None,
        })
        .collect())
    }
}

impl TranslationProvider for ClaudeTranslationProvider {}
