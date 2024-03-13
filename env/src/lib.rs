use std::env;

pub fn require_env_var(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("Missing required environment variable {name}."))
}

pub fn require_env_var_or<Error>(name: &str, error: Error) -> Result<String, Error> {
    env::var(name).map_err(|_| error)
}

pub fn env_var_or_default(name: &str, default: String) -> String {
    env::var(name).unwrap_or(default)
}
