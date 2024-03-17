use crate::error::ClaudeError;
use crate::model::ClaudeModel;
use reqwest::StatusCode;
use reqwest_retry_after::RetryAfterMiddleware;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Semaphore;

const API: &str = "https://api.anthropic.com/v1";

pub struct ClaudeClient {
    model: ClaudeModel,
    parallel_requests_semaphore: Semaphore,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl ClaudeClient {
    pub fn try_new(
        model: ClaudeModel,
        api_key: String,
        api_version: String,
        max_parallel_requests: usize,
    ) -> Result<Self, ClaudeError> {
        if max_parallel_requests == 0 {
            return Err(ClaudeError::InvalidMaxParallelRequestConfig);
        }

        let mut authentication_value = reqwest::header::HeaderValue::try_from(api_key)?;
        authentication_value.set_sensitive(true);

        let api_version = reqwest::header::HeaderValue::try_from(api_version)?;
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert("x-api-key", authentication_value);
        default_headers.insert("anthropic-version", api_version);

        let client = reqwest::ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        let client_with_middleware = reqwest_middleware::ClientBuilder::new(client)
            .with(RetryAfterMiddleware::new())
            .build();

        Ok(ClaudeClient {
            model,
            parallel_requests_semaphore: Semaphore::new(max_parallel_requests),
            client: client_with_middleware,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ClaudeContent {
    #[serde(rename = "text")]
    text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
}

impl ClaudeClient {
    pub async fn respond_to(
        &self,
        prompt: String,
        max_tokens: Option<usize>,
    ) -> Result<String, ClaudeError> {
        let _request_permit = self.parallel_requests_semaphore.acquire().await?;

        let url = format!("{API}/messages");
        let request_json = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": max_tokens.unwrap_or(4096),
        });

        let response = self.client.post(&url).json(&request_json).send().await?;
        let status = response.status();
        match status {
            StatusCode::OK => Ok(response
                .json::<ClaudeResponse>()
                .await?
                .content
                .first()
                .ok_or(ClaudeError::UnexpectedApiResponse(
                    "Expected response from claude but there wasn't one.".to_owned(),
                ))?
                .clone()
                .text),
            _ => Err(ClaudeError::UnexpectedApiResponse(
                format!("Expected 200 from {url} but got {status}").to_owned(),
            )),
        }
    }
}
