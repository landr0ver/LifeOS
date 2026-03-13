use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaudeMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
}

#[derive(Debug, Deserialize)]
struct ClaudeContent {
    text: String,
}

pub async fn chat(
    api_key: &str,
    messages: Vec<ClaudeMessage>,
    system_prompt: Option<&str>,
) -> Result<String, AppError> {
    let client = Client::new();

    let mut request = serde_json::json!({
        "model": "claude-sonnet-4-20250514",
        "max_tokens": 1024,
        "messages": messages,
    });

    if let Some(system) = system_prompt {
        request["system"] = serde_json::Value::String(system.to_string());
    }

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Claude API request failed: {}", e)))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::Internal(format!(
            "Claude API error {}: {}",
            status, body
        )));
    }

    let claude_response: ClaudeResponse = response
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse Claude response: {}", e)))?;

    Ok(claude_response
        .content
        .first()
        .map(|c| c.text.clone())
        .unwrap_or_default())
}
