use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::github::*;

pub async fn list_repos(pool: &PgPool) -> Result<Vec<GithubRepo>, AppError> {
    let repos = sqlx::query_as::<_, GithubRepo>(
        "SELECT id, owner, name, last_synced_at FROM github_repos ORDER BY name",
    )
    .fetch_all(pool)
    .await?;
    Ok(repos)
}

pub async fn connect_repo(pool: &PgPool, input: ConnectRepo, token: &str) -> Result<GithubRepo, AppError> {
    let repo = sqlx::query_as::<_, GithubRepo>(
        "INSERT INTO github_repos (owner, name, github_token_encrypted) \
         VALUES ($1, $2, $3) \
         ON CONFLICT (owner, name) DO UPDATE SET github_token_encrypted = $3 \
         RETURNING id, owner, name, last_synced_at",
    )
    .bind(&input.owner)
    .bind(&input.name)
    .bind(token)
    .fetch_one(pool)
    .await?;
    Ok(repo)
}

pub async fn list_issues(pool: &PgPool) -> Result<Vec<GithubIssue>, AppError> {
    let issues = sqlx::query_as::<_, GithubIssue>(
        "SELECT * FROM github_issues ORDER BY synced_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(issues)
}

pub async fn list_prs(pool: &PgPool) -> Result<Vec<GithubPullRequest>, AppError> {
    let prs = sqlx::query_as::<_, GithubPullRequest>(
        "SELECT * FROM github_pull_requests ORDER BY synced_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(prs)
}

/// Parse feature.md and return features that haven't been synced yet
pub fn parse_feature_md(content: &str) -> Vec<ParsedFeature> {
    let mut features = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("## ") {
            // Parse: ## [Tag] Title
            if rest.starts_with("[DONE]") {
                continue;
            }
            if let Some(bracket_end) = rest.find("] ") {
                let tag = &rest[1..bracket_end];
                let title = &rest[bracket_end + 2..];
                features.push(ParsedFeature {
                    tag: tag.to_string(),
                    title: title.to_string(),
                    description: String::new(),
                });
            }
        } else if !line.starts_with('#') && !line.is_empty() {
            // This is a description line for the last feature
            if let Some(last) = features.last_mut() {
                if !last.description.is_empty() {
                    last.description.push('\n');
                }
                last.description.push_str(line);
            }
        }
    }

    features
}

#[derive(Debug)]
pub struct ParsedFeature {
    pub tag: String,
    pub title: String,
    pub description: String,
}

pub async fn get_synced_features(pool: &PgPool, repo_id: Uuid) -> Result<Vec<String>, AppError> {
    let titles = sqlx::query_scalar::<_, String>(
        "SELECT feature_title FROM feature_sync_log WHERE repo_id = $1",
    )
    .bind(repo_id)
    .fetch_all(pool)
    .await?;
    Ok(titles)
}

pub async fn log_feature_sync(
    pool: &PgPool,
    repo_id: Uuid,
    title: &str,
    issue_number: i32,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO feature_sync_log (feature_title, issue_number, repo_id) VALUES ($1, $2, $3)",
    )
    .bind(title)
    .bind(issue_number)
    .bind(repo_id)
    .execute(pool)
    .await?;
    Ok(())
}
