use thiserror::Error;
use tokio::sync::AcquireError;

#[derive(Error, Debug)]
pub enum ClaudeError {
    #[error("Parallel request semaphore closed")]
    ParallelRequestSemaphoreClosed(#[from] AcquireError),
    #[error("Invalid max_parallel_requests config")]
    InvalidMaxParallelRequestConfig,
    #[error("InvalidHeaderValue {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("ReqwestError {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("ReqwestMiddlewareError {0}")]
    ReqwestMiddlewareError(#[from] reqwest_middleware::Error),
    #[error("ReqwestMiddlewareError {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Unexpected API response {0}")]
    UnexpectedApiResponse(String),
    #[error("Invalid api key")]
    InvalidApiKey,
    #[error("Invalid model")]
    InvalidModel,
}
