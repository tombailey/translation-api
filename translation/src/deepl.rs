use crate::{
    HealthCheck, Language, Translation, TranslationError, TranslationInput, TranslationOutput,
    TranslationProvider,
};
use async_trait::async_trait;
use deepl::client::DeepLClient;
use futures::future::join_all;
use futures::TryFutureExt;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

pub struct DeepLTranslationProvider {
    deepl_client: DeepLClient,
}

impl DeepLTranslationProvider {
    pub fn new(deepl_client: DeepLClient) -> Self {
        DeepLTranslationProvider { deepl_client }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
struct LanguageTranslationPair {
    source_lang: Option<String>,
    target_lang: String,
}

impl From<TranslationInput> for LanguageTranslationPair {
    fn from(translation_input: TranslationInput) -> Self {
        LanguageTranslationPair {
            source_lang: translation_input
                .source_language
                .map(|source| Language::to_string(&source)),
            target_lang: translation_input.target_language.to_string(),
        }
    }
}

#[async_trait]
impl HealthCheck for DeepLTranslationProvider {
    async fn is_healthy(&self) -> Option<bool> {
        Some(
            self.deepl_client
                .get_usage()
                .await
                .map(|usage| usage.character_limit > usage.character_count)
                .ok()
                .unwrap_or(false),
        )
    }
}

impl Translation for DeepLTranslationProvider {
    async fn translate(
        &self,
        inputs: Vec<TranslationInput>,
    ) -> Result<Vec<TranslationOutput>, TranslationError> {
        let language_pair_to_inputs = inputs
            .into_iter()
            .zip(0_u32..)
            .map(|(translation_input, index)| {
                (
                    LanguageTranslationPair::from(translation_input.clone()),
                    (translation_input.text, index),
                )
            })
            .fold(
                HashMap::<LanguageTranslationPair, Vec<(String, u32)>>::new(),
                |mut pair_to_inputs, (language_pair, input)| {
                    pair_to_inputs.entry(language_pair).or_default().push(input);
                    pair_to_inputs
                },
            );

        let translations = language_pair_to_inputs
            .into_iter()
            .map(|(language_pair, inputs)| {
                let texts = inputs.iter().cloned().map(|(text, _)| text).collect_vec();
                let indexes = inputs.iter().cloned().map(|(_, index)| index).collect_vec();
                self.deepl_client
                    .translate(texts, language_pair.source_lang, language_pair.target_lang)
                    .and_then(|translations| async move {
                        let source_language = translations.source_language.clone();
                        Ok(translations
                            .texts
                            .into_iter()
                            .zip(indexes)
                            .map(|(text, index)| (text, index, source_language.clone()))
                            .collect_vec())
                    })
            })
            .collect_vec();

        let mut all_results = Vec::<(String, u32, Option<String>)>::new();
        for result in join_all(translations).await {
            all_results.extend(result?);
        }

        Ok(all_results
            .into_iter()
            .sorted_by(|first, second| first.1.cmp(&second.1))
            .map(|(text, _, source_language)| TranslationOutput {
                text,
                source_language: match source_language {
                    None => None,
                    Some(source) => Language::from_str(source.to_ascii_lowercase().as_str()).ok(),
                },
            })
            .collect_vec())
    }
}

impl TranslationProvider for DeepLTranslationProvider {}
