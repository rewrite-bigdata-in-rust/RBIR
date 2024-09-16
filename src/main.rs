mod markdown;
mod spec;
mod utils;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let data = spec::load().await?;
    markdown::render(data).await?;

    Ok(())
}
