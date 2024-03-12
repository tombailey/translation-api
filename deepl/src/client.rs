use crate::error::DeepLError;
use futures::future::join_all;
use itertools::Itertools;
use reqwest::StatusCode;
use reqwest_retry_after::RetryAfterMiddleware;
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

pub struct DeepLClient {
    api: String,
    parallel_requests_semaphore: Semaphore,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl DeepLClient {
    pub fn try_new(
        api: String,
        authentication_key: String,
        max_parallel_requests: usize,
    ) -> Result<Self, DeepLError> {
        if max_parallel_requests == 0 {
            return Err(DeepLError::InvalidMaxParallelRequestConfig);
        }

        let mut authentication_value = reqwest::header::HeaderValue::try_from(format!(
            "DeepL-Auth-Key {}",
            authentication_key
        ))?;
        authentication_value.set_sensitive(true);

        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert("Authorization", authentication_value);

        let client = reqwest::ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        let client_with_middleware = reqwest_middleware::ClientBuilder::new(client)
            .with(RetryAfterMiddleware::new())
            .build();

        Ok(DeepLClient {
            api,
            parallel_requests_semaphore: Semaphore::new(max_parallel_requests),
            client: client_with_middleware,
        })
    }
}

#[derive(Deserialize, Serialize)]
struct DeepLTranslationRequest {
    #[serde(rename = "text")]
    texts: Vec<String>,
    #[serde(rename = "source_lang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    source_language: Option<String>,
    #[serde(rename = "target_lang")]
    target_language: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct DeepLTranslation {
    text: String,
    #[serde(rename = "detected_source_language")]
    source_language: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct DeepLTranslationResponse {
    translations: Vec<DeepLTranslation>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeepLTranslationOutput {
    pub texts: Vec<String>,
    pub source_language: Option<String>,
}

impl From<DeepLTranslationResponse> for DeepLTranslationOutput {
    fn from(value: DeepLTranslationResponse) -> Self {
        let texts = value
            .translations
            .clone()
            .into_iter()
            .map(|translation| translation.text)
            .collect_vec();

        let source_language = value
            .translations
            .first()
            .map(|translation| translation.source_language.clone());

        DeepLTranslationOutput {
            texts,
            source_language,
        }
    }
}

impl DeepLClient {
    async fn translate_batch(
        &self,
        texts: Vec<String>,
        source_language: Option<String>,
        target_language: String,
    ) -> Result<DeepLTranslationOutput, DeepLError> {
        let _request_permit = self.parallel_requests_semaphore.acquire().await?;

        let url = format!("{}/translate", self.api);
        let response = self
            .client
            .post(url.clone())
            .json(&DeepLTranslationRequest {
                texts,
                source_language,
                target_language,
            })
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(DeepLTranslationOutput::from(
                response.json::<DeepLTranslationResponse>().await?,
            )),
            _ => Err(DeepLError::UnexpectedApiResponse(
                format!("Expected 200 from {url} but got {status}").to_owned(),
            )),
        }
    }
}

const MAX_TEXTS_PER_REQUEST: u8 = 50;

impl DeepLClient {
    pub async fn translate(
        &self,
        texts: Vec<String>,
        source_language: Option<String>,
        target_language: String,
    ) -> Result<DeepLTranslationOutput, DeepLError> {
        let translation_futures = texts
            .chunks(texts.len().div_ceil(MAX_TEXTS_PER_REQUEST as usize))
            .map(|chunk| {
                self.translate_batch(
                    chunk.to_vec(),
                    source_language.clone(),
                    target_language.clone(),
                )
            })
            .collect_vec();

        let mut translated_texts = Vec::<String>::new();
        let mut source_language: Option<String> = None;
        for batch_result in join_all(translation_futures).await {
            let result = batch_result?.clone();
            source_language = source_language.or(result.source_language);
            translated_texts.extend(result.texts);
        }

        Ok(DeepLTranslationOutput {
            texts: translated_texts,
            source_language,
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct DeepLUsage {
    pub character_count: u64,
    pub character_limit: u64,
}

impl DeepLClient {
    pub async fn get_usage(&self) -> Result<DeepLUsage, DeepLError> {
        let _request_permit = self.parallel_requests_semaphore.acquire().await?;

        let url = format!("{}/usage", self.api);
        let response = self.client.get(&url).send().await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json::<DeepLUsage>().await?),
            _ => Err(DeepLError::UnexpectedApiResponse(
                format!("Expected 200 from {url} but got {status}").to_owned(),
            )),
        }
    }
}
