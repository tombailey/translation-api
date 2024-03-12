use crate::dependency::translation::claude::maybe_create_claude_translation_provider;
use crate::dependency::translation::deepl::maybe_create_deepl_translation_provider;
use async_trait::async_trait;
use translation::claude::ClaudeTranslationProvider;
use translation::deepl::DeepLTranslationProvider;
use translation::{
    HealthCheck, Translation, TranslationError, TranslationInput, TranslationOutput,
    TranslationProvider,
};

pub mod claude;
pub mod deepl;

pub fn get_first_configured_translator() -> Option<Translator> {
    let claude = maybe_create_claude_translation_provider().map(Translator::Claude);
    let deepl = maybe_create_deepl_translation_provider().map(Translator::DeepL);
    claude.or(deepl)
}

pub enum Translator {
    Claude(ClaudeTranslationProvider),
    DeepL(DeepLTranslationProvider),
}

impl Translation for Translator {
    async fn translate(
        &self,
        inputs: Vec<TranslationInput>,
    ) -> Result<Vec<TranslationOutput>, TranslationError> {
        match self {
            Translator::Claude(claude) => claude.translate(inputs).await,
            Translator::DeepL(deepl) => deepl.translate(inputs).await,
        }
    }
}

#[async_trait]
impl HealthCheck for Translator {
    async fn is_healthy(&self) -> Option<bool> {
        match self {
            Translator::Claude(claude) => claude.is_healthy().await,
            Translator::DeepL(deepl) => deepl.is_healthy().await,
        }
    }
}

impl TranslationProvider for Translator {}
