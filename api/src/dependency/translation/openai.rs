use openai::client::{OpenAIClient, OpenAIModel};
use env::require_env_var;
use translation::openai::OpenAITranslationProvider;

pub const OPENAI_MODEL: &str = "OPENAI_MODEL";
pub const OPENAI_API_KEY: &str = "OPENAI_API_KEY";
pub const OPENAI_MAX_PARALLEL_REQUESTS: &str = "OPENAI_MAX_PARALLEL_REQUESTS";

pub fn maybe_create_openai_translation_provider() -> Option<OpenAITranslationProvider> {
    let model = OpenAIModel::try_from(require_env_var(OPENAI_MODEL).ok()?).ok()?;
    let api_key = require_env_var(OPENAI_API_KEY).ok()?;
    let max_parallel_requests = require_env_var(OPENAI_MAX_PARALLEL_REQUESTS)
        .ok()?
        .parse::<usize>()
        .ok()?;

    let client = OpenAIClient::try_new(model, api_key, max_parallel_requests).ok()?;
    Some(OpenAITranslationProvider::new(client))
}
