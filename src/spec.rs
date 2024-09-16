use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;

/// Load data from `data.toml`.
pub async fn load() -> Result<Data> {
    let content = read_to_string("data.toml").await?;
    let mut data: Data = toml::from_str(&content)?;

    // Sort data for better display.
    data.library.sort();
    data.project.sort();
    data.post.sort();

    Ok(data)
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(deny_unknown_fields)]
pub struct Data {
    pub project: Vec<Project>,
    pub library: Vec<Library>,
    pub post: Vec<Post>,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub name: String,
    pub repo: String,
    pub stars: Option<String>,
    pub description: String,
}

impl<'de> Deserialize<'de> for Project {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ProjectData {
            name: String,
            repo: String,
            description: String,
        }

        let data = ProjectData::deserialize(deserializer)?;
        let stars = create_stars_url(&data.repo);

        Ok(Project {
            name: data.name,
            repo: data.repo,
            description: data.description,
            stars,
        })
    }
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(deny_unknown_fields)]
pub struct Library {
    pub name: String,
    pub repo: String,
    pub description: String,
    pub stars: Option<String>,
    pub path: Option<String>,
}

impl<'de> Deserialize<'de> for Library {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct LibraryData {
            name: String,
            repo: String,
            description: String,
            path: Option<String>,
        }

        let data = LibraryData::deserialize(deserializer)?;
        let stars = create_stars_url(&data.repo);

        Ok(Library {
            name: data.name,
            repo: data.repo,
            description: data.description,
            path: data.path,
            stars,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(deny_unknown_fields)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub link: String,
}

fn create_stars_url(repo: &str) -> Option<String> {
    // removing the trailing / if it exists
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_load_project() -> Result<()> {
        let content = r#"
[[project]]
description = "RBIR stands for Rewrite Bigdata in Rust. RBIR aims to create a big data ecosystem using Rust."
name = "RBIR"
repo = "https://github.com/rewrite-bigdata-in-rust/RBIR"

[[project]]
description = "A project where stars should be None"
name = "No Stars Project"
repo = "https://gitlab.com/project/project"

[[project]]
description = "A project with a trailing slash"
name = "Trailing slash Project"
repo = "https://github.com/project/project/"

[[library]]
description = "A unified data access layer, empowering users to seamlessly and efficiently retrieve data from diverse storage services."
name = "Apache OpenDAL"
repo = "https://github.com/apache/opendal"

[[library]]
description = "A library where stars should be None"
name = "No Stars Library"
repo = "https://gitlab.com/library/library"

[[post]]
author = "[@Xuanwo](https://github.com/Xuanwo)"
link = "https://xuanwo.io/2024/07-rewrite-bigdata-in-rust/"
title = "Rewrite Bigdata in Rust"
            "#;
        let input: Data = toml::from_str(content)?;

        let expected = Data {
            project:vec![
                Project{
                    description:"RBIR stands for Rewrite Bigdata in Rust. RBIR aims to create a big data ecosystem using Rust.".to_string(),
                    name:"RBIR".to_string(),
                    repo:"https://github.com/rewrite-bigdata-in-rust/RBIR".to_string(),
                    stars: Some("https://img.shields.io/github/stars/rewrite-bigdata-in-rust/RBIR".to_string()),
                },
                Project{
                    description:"A project where stars should be None".to_string(),
                    name:"No Stars Project".to_string(),
                    repo:"https://gitlab.com/project/project".to_string(),
                    stars: None,
                },
                Project{
                    description:"A project with a trailing slash".to_string(),
                    name:"Trailing slash Project".to_string(),
                    repo:"https://github.com/project/project/".to_string(),
                    stars: Some("https://img.shields.io/github/stars/project/project".to_string()),
                }],
            library: vec![
                Library{
                    description: "A unified data access layer, empowering users to seamlessly and efficiently retrieve data from diverse storage services.".to_string(),
                    name:"Apache OpenDAL".to_string(),
                    repo:"https://github.com/apache/opendal".to_string(),
                    stars: Some("https://img.shields.io/github/stars/apache/opendal".to_string()),
                    path: None,
                },
                Library{
                    description: "A library where stars should be None".to_string(),
                    name:"No Stars Library".to_string(),
                    repo:"https://gitlab.com/library/library".to_string(),
                    stars: None,
                    path: None,
                }],
            post:vec![Post {
                author:"[@Xuanwo](https://github.com/Xuanwo)".to_string(),
                link: "https://xuanwo.io/2024/07-rewrite-bigdata-in-rust/".to_string(),
                title: "Rewrite Bigdata in Rust".to_string()
            }]
        };

        println!("{:?}", input);
        assert_eq!(input, expected);

        Ok(())
    }
}
