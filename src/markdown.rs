use crate::spec::{Data, Library, Project};
use crate::utils;
use anyhow::{anyhow, Result};
use octocrab::models::repos::Release;
use octocrab::models::License;
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tokio::fs::write;

pub async fn render(data: Data) -> Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let github = Octocrab::builder().personal_token(token).build()?;

    let mut tera = Tera::new("templates/*.tmpl")?;
    tera.register_filter("to_snake_case", utils::to_snake_case_filter);
    tera.register_filter("stars_url", utils::stars_url_filter);

    render_readme(&tera, &data).await?;
    render_projects(&github, &tera, &data).await?;
    render_libraries(&github, &tera, &data).await?;

    Ok(())
}

async fn render_readme(tera: &Tera, data: &Data) -> Result<()> {
    let context = Context::from_serialize(data)?;

    let content = tera.render("README.tmpl", &context)?;
    write("README.md", content.as_bytes()).await?;
    Ok(())
}

async fn render_projects(github: &Octocrab, tera: &Tera, data: &Data) -> Result<()> {
    for project in &data.project {
        render_project(github, tera, project).await?;
    }
    Ok(())
}

async fn render_project(github: &Octocrab, tera: &Tera, project: &Project) -> Result<()> {
    let repo_status = fetch_repo_status(github, &project.repo).await?;

    let mut context = Context::from_serialize(project)?;
    context.insert("status", &repo_status);

    let content = tera.render("project.tmpl", &context)?;
    write(
        format!("projects/{}.md", utils::to_snake_case(&project.name)),
        content.as_bytes(),
    )
    .await?;
    Ok(())
}

async fn render_libraries(github: &Octocrab, tera: &Tera, data: &Data) -> Result<()> {
    for library in &data.library {
        render_library(github, tera, library).await?;
    }
    Ok(())
}

async fn render_library(github: &Octocrab, tera: &Tera, library: &Library) -> Result<()> {
    let repo_status = fetch_repo_status(github, &library.repo).await?;

    let mut context = Context::from_serialize(library)?;
    context.insert("status", &repo_status);

    let content = tera.render("library.tmpl", &context)?;
    write(
        format!("libraries/{}.md", utils::to_snake_case(&library.name)),
        content.as_bytes(),
    )
    .await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RepoStatus {
    license: Option<License>,
    latest_release: Option<Release>,
}

async fn fetch_repo_status(github: &Octocrab, repo: &str) -> Result<RepoStatus> {
    let parts: Vec<&str> = repo.trim_end_matches('/').split('/').collect();
    if parts.len() < 2 {
        return Err(anyhow!("Invalid repo url: {repo}"));
    }
    let owner = parts[parts.len() - 2].to_string();
    let name = parts[parts.len() - 1].to_string();

    let repo_handler = github.repos(&owner, &name);

    let repo = repo_handler.get().await?;
    let license = repo.license;
    let latest_release = repo_handler.releases().get_latest().await.ok();

    Ok(RepoStatus {
        license,
        latest_release,
    })
}
