use serde::{Deserialize, Serialize};
use crate::error::ClaudeError;

#[derive(Deserialize, Serialize)]
pub enum ClaudeModel {
    #[serde(rename = "claude-3-opus-20240229")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet-20240229")]
    Claude3Sonnet,
    #[serde(rename = "claude-2.1")]
    Claude2Point1,
    #[serde(rename = "claude-2.0")]
    Claude2Point0,
    #[serde(rename = "claude-instant-1.2")]
    ClaudeInstant1Point2,
}

impl TryFrom<String> for ClaudeModel {
    type Error = ClaudeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_ref() {
            "claude-3-opus-20240229" => Ok(ClaudeModel::Claude3Opus),
            "claude-3-sonnet-20240229" => Ok(ClaudeModel::Claude3Sonnet),
            "claude-2.1" => Ok(ClaudeModel::Claude2Point1),
            "claude-2.0" => Ok(ClaudeModel::Claude2Point0),
            "claude-instant-1.2" => Ok(ClaudeModel::ClaudeInstant1Point2),
            _ => Err(ClaudeError::InvalidModel),
        }
    }
}
