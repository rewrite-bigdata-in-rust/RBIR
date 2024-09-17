use std::collections::HashMap;
use tera::Value;

/// Convert given string to snake case.
///
/// - `Apache OpenDAL` => `apache_opendal`
/// - `LanceDB` => `lancedb`
pub fn to_snake_case(input: &str) -> String {
    input.to_lowercase().replace(" ", "_")
}

pub fn to_snake_case_filter(v: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let input = v
        .as_str()
        .ok_or_else(|| tera::Error::msg("The value is not a string"))?;
    Ok(to_snake_case(input).into())
}

fn create_stars_url(repo: &str) -> Option<String> {
    let repo = repo.strip_suffix('/').unwrap_or(repo);

    // Parse the repo field to extract owner and repo name for stars URL.
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() < 4 || !parts[2].starts_with("github.com") {
        None
    } else {
        let owner = parts[parts.len() - 2].to_string();
        let repo_name = parts[parts.len() - 1].to_string();
        Some(format!(
            "https://img.shields.io/github/stars/{}/{}",
            owner, repo_name
        ))
    }
}

pub fn stars_url_filter(v: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let input = v
        .as_str()
        .ok_or_else(|| tera::Error::msg("The value is not a string"))?;
    Ok(create_stars_url(input).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_stars_url() {
        let repo = "https://github.com/rewrite-bigdata-in-rust/RBIR";
        let result = create_stars_url(repo);

        assert_eq!(
            result,
            Some("https://img.shields.io/github/stars/rewrite-bigdata-in-rust/RBIR".to_string())
        );
    }

    #[test]
    fn test_create_stars_url_none() {
        let repo = "https://gitlab.com/project/project";
        let result = create_stars_url(repo);

        assert_eq!(result, None);
    }

    #[test]
    fn test_create_stars_url_trailing_slash() {
        let repo = "https://github.com/project/project/";
        let result = create_stars_url(repo);

        assert_eq!(
            result,
            Some("https://img.shields.io/github/stars/project/project".to_string())
        );
    }
}
