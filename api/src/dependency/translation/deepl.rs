use deepl::client::DeepLClient;
use env::require_env_var;
use translation::deepl::DeepLTranslationProvider;

pub const DEEPL_API: &str = "DEEPL_API";
pub const DEEPL_AUTHENTICATION_KEY: &str = "DEEPL_AUTHENTICATION_KEY";
pub const DEEPL_MAX_PARALLEL_REQUESTS: &str = "DEEPL_MAX_PARALLEL_REQUESTS";

pub fn maybe_create_deepl_translation_provider() -> Option<DeepLTranslationProvider> {
    let api = require_env_var(DEEPL_API).ok()?;
    let authentication_key = require_env_var(DEEPL_AUTHENTICATION_KEY).ok()?;
    let max_parallel_requests = require_env_var(DEEPL_MAX_PARALLEL_REQUESTS)
        .ok()?
        .parse::<usize>()
        .ok()?;

    let client = DeepLClient::try_new(api, authentication_key, max_parallel_requests).ok()?;
    Some(DeepLTranslationProvider::new(client))
}
