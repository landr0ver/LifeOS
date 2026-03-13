use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub claude_api_key: Option<String>,
    pub github_token: Option<String>,
    pub auth_password: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            claude_api_key: env::var("CLAUDE_API_KEY").ok(),
            github_token: env::var("GITHUB_TOKEN").ok(),
            auth_password: env::var("AUTH_PASSWORD").unwrap_or_else(|_| "changeme".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
        }
    }
}
