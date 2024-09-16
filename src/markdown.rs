use crate::spec::Data;
use anyhow::Result;
use std::fs::write;
use tera::{Context, Tera};

pub fn render(data: Data) -> Result<String> {
    let tera = Tera::new("templates/*.tmpl")?;
    let context = Context::from_serialize(data)?;

    let content = tera.render("README.md.tmpl", &context)?;
    write("README.md", content.as_bytes())?;
    Ok(content)
}
