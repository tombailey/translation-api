use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Debug, Deserialize_enum_str, Serialize_enum_str, PartialEq, Eq)]
pub enum OpenAIModel {
    #[serde(rename = "gpt-4o-mini")]
    GPT4OMini,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    GPT4OMini20240718,
    #[serde(rename = "gpt-4o")]
    GPT4O,
    #[serde(rename = "gpt-4o-2024-05-13")]
    GPT4O20240513,
    #[serde(rename = "gpt-4-turbo-preview")]
    GPT4TurboPreview,
    #[serde(rename = "gpt-4-0125-preview")]
    GPT40125Preview,
    #[serde(rename = "gpt-4")]
    GPT4,
    #[serde(rename = "gpt-4-32k")]
    GPT432K,
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
            OpenAIModel::try_from("gpt-4-32k".to_owned()).unwrap(),
            OpenAIModel::GPT432K
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
