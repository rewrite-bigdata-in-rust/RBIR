mod markdown;
mod spec;

use anyhow::Result;

fn main() -> Result<()> {
    let data = spec::load()?;
    let content = markdown::render(data)?;
    println!("{}", content);

    Ok(())
}
