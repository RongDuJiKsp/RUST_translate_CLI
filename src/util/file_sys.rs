use anyhow::Context;
use std::fs;
use std::path::Path;

pub fn create_parent_dir(path: &str) -> anyhow::Result<()> {
    let parent_dir = Path::new(path)
        .parent()
        .with_context(|| format!("parent directory of `{}` invalid", path))?;
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir)?;
    }
    Ok(())
}