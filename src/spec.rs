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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub name: String,
    pub repo: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(deny_unknown_fields)]
pub struct Library {
    pub name: String,
    pub repo: String,
    pub description: String,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(deny_unknown_fields)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub link: String,
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

[[library]]
description = "A unified data access layer, empowering users to seamlessly and efficiently retrieve data from diverse storage services."
name = "Apache OpenDAL"
repo = "https://github.com/apache/opendal"

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
                    repo:"https://github.com/rewrite-bigdata-in-rust/RBIR".to_string()
                }],
            library: vec![
                Library{
                    description: "A unified data access layer, empowering users to seamlessly and efficiently retrieve data from diverse storage services.".to_string(),
                    name:"Apache OpenDAL".to_string(),
                    repo:"https://github.com/apache/opendal".to_string(),
                    path: None
                }],
            post:vec![Post {
                author:"[@Xuanwo](https://github.com/Xuanwo)".to_string(),
                link: "https://xuanwo.io/2024/07-rewrite-bigdata-in-rust/".to_string(),
                title: "Rewrite Bigdata in Rust".to_string()
            }]
        };

        assert_eq!(input, expected);

        Ok(())
    }
}
