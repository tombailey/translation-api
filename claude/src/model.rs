use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
#[derive(Debug, Deserialize_enum_str, Serialize_enum_str, PartialEq, Eq)]
pub enum ClaudeModel {
    #[serde(rename = "claude-opus-4-1-20250805")]
    Claude4Point1Opus20250805,
    #[serde(rename = "claude-opus-4-20250514")]
    Claude4Opus20250805,
    #[serde(rename = "claude-sonnet-4-20250514")]
    Claude4Sonnet20250514,
    #[serde(rename = "claude-3-7-sonnet-20250219")]
    Claude3Point7Sonnet20250219,
    #[serde(rename = "claude-3-5-sonnet-20241022")]
    Claude3Point5Sonnet20241022,
    #[serde(rename = "claude-3-5-haiku-20241022")]
    Claude3Point5Haiku20241022,
    #[serde(rename = "claude-3-5-sonnet-20240620")]
    Claude3Point5Sonnet,
    #[serde(rename = "claude-3-opus-20240229")]
    Claude3Opus,
    #[serde(rename = "claude-3-haiku-20240307")]
    Claude3Haiku,
}
#[cfg(test)]
mod tests {
    use crate::model::ClaudeModel;
    #[test]
    fn it_should_get_models_from_string() {
        assert_eq!(
            ClaudeModel::try_from("claude-opus-4-1-20250805".to_owned()).unwrap(),
            ClaudeModel::Claude4Point1Opus20250805
        );
        assert_eq!(
            ClaudeModel::try_from("claude-opus-4-20250514".to_owned()).unwrap(),
            ClaudeModel::Claude4Opus20250805
        );
        assert_eq!(
            ClaudeModel::try_from("claude-sonnet-4-20250514".to_owned()).unwrap(),
            ClaudeModel::Claude4Sonnet20250514
        );

        assert_eq!(
            ClaudeModel::try_from("claude-3-7-sonnet-20250219".to_owned()).unwrap(),
            ClaudeModel::Claude3Point7Sonnet20250219
        );
        assert_eq!(
            ClaudeModel::try_from("claude-3-5-sonnet-20241022".to_owned()).unwrap(),
            ClaudeModel::Claude3Point5Sonnet20241022
        );
        assert_eq!(
            ClaudeModel::try_from("claude-3-5-haiku-20241022".to_owned()).unwrap(),
            ClaudeModel::Claude3Point5Haiku20241022
        );
        assert_eq!(
            ClaudeModel::try_from("claude-3-5-sonnet-20240620".to_owned()).unwrap(),
            ClaudeModel::Claude3Point5Sonnet
        );
        assert_eq!(
            ClaudeModel::try_from("claude-3-opus-20240229".to_owned()).unwrap(),
            ClaudeModel::Claude3Opus
        );
        assert_eq!(
            ClaudeModel::try_from("claude-3-haiku-20240307".to_owned()).unwrap(),
            ClaudeModel::Claude3Haiku
        );
    }
}
