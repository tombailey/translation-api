use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Debug, Deserialize_enum_str, Serialize_enum_str, PartialEq, Eq)]
pub enum ClaudeModel {
    #[serde(rename = "claude-3-opus-20240229")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet-20240229")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-haiku-20240307")]
    Claude3Haiku,
    #[serde(rename = "claude-2.1")]
    Claude2Point1,
    #[serde(rename = "claude-2.0")]
    Claude2Point0,
    #[serde(rename = "claude-instant-1.2")]
    ClaudeInstant1Point2,
}

#[cfg(test)]
mod tests {
    use crate::model::ClaudeModel;

    #[test]
    fn it_should_get_models_from_string() {
        assert_eq!(
            ClaudeModel::try_from("claude-3-opus-20240229".to_owned()).unwrap(),
            ClaudeModel::Claude3Opus
        );
        assert_eq!(
            ClaudeModel::try_from("claude-3-sonnet-20240229".to_owned()).unwrap(),
            ClaudeModel::Claude3Sonnet
        );
        assert_eq!(
            ClaudeModel::try_from("claude-2.1".to_owned()).unwrap(),
            ClaudeModel::Claude2Point1
        );
        assert_eq!(
            ClaudeModel::try_from("claude-2.0".to_owned()).unwrap(),
            ClaudeModel::Claude2Point0
        );
        assert_eq!(
            ClaudeModel::try_from("claude-instant-1.2".to_owned()).unwrap(),
            ClaudeModel::ClaudeInstant1Point2
        );
    }
}
