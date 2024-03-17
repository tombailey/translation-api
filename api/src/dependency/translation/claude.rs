use claude::client::ClaudeClient;
use claude::model::ClaudeModel;
use env::require_env_var;
use translation::claude::ClaudeTranslationProvider;

pub const CLAUDE_MODEL: &str = "CLAUDE_MODEL";
pub const CLAUDE_API_KEY: &str = "CLAUDE_API_KEY";
pub const CLAUDE_API_VERSION: &str = "CLAUDE_API_VERSION";
pub const CLAUDE_MAX_PARALLEL_REQUESTS: &str = "CLAUDE_MAX_PARALLEL_REQUESTS";

pub fn maybe_create_claude_translation_provider() -> Option<ClaudeTranslationProvider> {
    let model = ClaudeModel::try_from(require_env_var(CLAUDE_MODEL).ok()?).ok()?;
    let api_key = require_env_var(CLAUDE_API_KEY).ok()?;
    let api_version = require_env_var(CLAUDE_API_VERSION).ok()?;
    let max_parallel_requests = require_env_var(CLAUDE_MAX_PARALLEL_REQUESTS)
        .ok()?
        .parse::<usize>()
        .ok()?;

    let client = ClaudeClient::try_new(model, api_key, api_version, max_parallel_requests).ok()?;
    Some(ClaudeTranslationProvider::new(client))
}
