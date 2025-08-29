use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
#[derive(Debug, Deserialize_enum_str, Serialize_enum_str, PartialEq, Eq)]
pub enum OpenAIModel {
    #[serde(rename = "gpt-5-2025-08-07")]
    GPT520250807,
    #[serde(rename = "gpt-5-mini-2025-08-07")]
    GPT5Mini20250807,
    #[serde(rename = "gpt-5-nano-2025-08-07")]
    GPT5Nano20250807,
    #[serde(rename = "gpt-4.1-mini-2025-04-14")]
    GPT4Point1Mini20250414,
    #[serde(rename = "gpt-4.1-nano-2025-04-14")]
    GPT4Point1Nano20250414,
    #[serde(rename = "gpt-4o-mini")]
    GPT4OMini,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    GPT4OMini20240718,
    #[serde(rename = "gpt-4o")]
    GPT4O,
    #[serde(rename = "gpt-4o-2024-05-13")]
    GPT4O20240513,
    #[serde(rename = "gpt-4o-2024-08-06")]
    GPT4O20240806,
    #[serde(rename = "gpt-4-turbo-preview")]
    GPT4TurboPreview,
    #[serde(rename = "gpt-4-0125-preview")]
    GPT40125Preview,
    #[serde(rename = "gpt-4")]
    GPT4,
    #[serde(rename = "gpt-3.5-turbo")]
    GPT3Point5Turbo,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    GPT3Point5Turbo1106,
}
#[cfg(test)]
mod tests {
    use crate::model::OpenAIModel;
    #[test]
    fn it_should_get_models_from_string() {
        assert_eq!(
            OpenAIModel::try_from("gpt-5-2025-08-07".to_owned()).unwrap(),
            OpenAIModel::GPT520250807
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-5-mini-2025-08-07".to_owned()).unwrap(),
            OpenAIModel::GPT5Mini20250807
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-5-nano-2025-08-07".to_owned()).unwrap(),
            OpenAIModel::GPT5Nano20250807
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-4.1-mini-2025-04-14".to_owned()).unwrap(),
            OpenAIModel::GPT4Point1Mini20250414
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-4.1-nano-2025-04-14".to_owned()).unwrap(),
            OpenAIModel::GPT4Point1Nano20250414
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-4o-mini".to_owned()).unwrap(),
            OpenAIModel::GPT4OMini
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-4o-mini-2024-07-18".to_owned()).unwrap(),
            OpenAIModel::GPT4OMini20240718
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-4o".to_owned()).unwrap(),
            OpenAIModel::GPT4O
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-4o-2024-05-13".to_owned()).unwrap(),
            OpenAIModel::GPT4O20240513
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-4o-2024-08-06".to_owned()).unwrap(),
            OpenAIModel::GPT4O20240806
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-4-turbo-preview".to_owned()).unwrap(),
            OpenAIModel::GPT4TurboPreview
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-4-0125-preview".to_owned()).unwrap(),
            OpenAIModel::GPT40125Preview
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-4".to_owned()).unwrap(),
            OpenAIModel::GPT4
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-3.5-turbo".to_owned()).unwrap(),
            OpenAIModel::GPT3Point5Turbo
        );
        assert_eq!(
            OpenAIModel::try_from("gpt-3.5-turbo-1106".to_owned()).unwrap(),
            OpenAIModel::GPT3Point5Turbo1106
        );
    }
}
