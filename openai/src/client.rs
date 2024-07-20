use crate::error::OpenAIError;
use crate::model::OpenAIModel;
use reqwest::StatusCode;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Semaphore;

pub struct OpenAIClient {
    model: OpenAIModel,
    parallel_requests_semaphore: Semaphore,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl OpenAIClient {
    pub fn try_new(
        model: OpenAIModel,
        api_key: String,
        max_parallel_requests: usize,
    ) -> Result<Self, OpenAIError> {
        if max_parallel_requests == 0 {
            return Err(OpenAIError::InvalidMaxParallelRequestConfig);
        }

        let mut api_key_value =
            reqwest::header::HeaderValue::try_from(format!("Bearer {}", api_key))?;
        api_key_value.set_sensitive(true);
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert("authorization", api_key_value);

        let client = reqwest::ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client_with_middleware = reqwest_middleware::ClientBuilder::new(client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Ok(OpenAIClient {
            model,
            parallel_requests_semaphore: Semaphore::new(max_parallel_requests),
            client: client_with_middleware,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct OpenAIMessageResponse {
    content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct OpenAIChoiceResponse {
    message: OpenAIMessageResponse,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct OpenAIChatResponse {
    choices: Vec<OpenAIChoiceResponse>,
}

const CHAT_COMPLETION_API_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

impl OpenAIClient {
    pub async fn respond_to(
        &self,
        system_prompt: String,
        prompt: String,
        max_tokens: Option<usize>,
    ) -> Result<String, OpenAIError> {
        let _request_permit = self.parallel_requests_semaphore.acquire().await?;

        let request_json = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": max_tokens.unwrap_or(4096),
        });

        let response = self
            .client
            .post(CHAT_COMPLETION_API_ENDPOINT)
            .json(&request_json)
            .send()
            .await?;
        let status = response.status();
        match status {
            StatusCode::OK => Ok(response
                .json::<OpenAIChatResponse>()
                .await?
                .choices
                .first()
                .ok_or(OpenAIError::UnexpectedApiResponse(
                    "Expected response from openai but there wasn't one.".to_owned(),
                ))?
                .clone()
                .message
                .content),
            _ => Err(OpenAIError::UnexpectedApiResponse(
                format!("Expected 200 from {CHAT_COMPLETION_API_ENDPOINT} but got {status}")
                    .to_owned(),
            )),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpenAIModelsResponse {
    #[serde(rename = "data")]
    models: Vec<OpenAIModelResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpenAIModelResponse {
    id: String,
}

const MODELS_API_ENDPOINT: &str = "https://api.openai.com/v1/models";

impl OpenAIClient {
    pub async fn get_models(&self) -> Result<OpenAIModelsResponse, OpenAIError> {
        let _request_permit = self.parallel_requests_semaphore.acquire().await?;

        let response = self.client.get(MODELS_API_ENDPOINT).send().await?;

        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.json::<OpenAIModelsResponse>().await?),
            _ => Err(OpenAIError::UnexpectedApiResponse(
                format!("Expected 200 from {MODELS_API_ENDPOINT} but got {status}").to_owned(),
            )),
        }
    }
}
