use std::path::PathBuf;

use crate::{database::sqlite, error::NoemaResult};

pub fn read_note(db: Option<PathBuf>, id: &str) -> NoemaResult {
    let content = sqlite::get_content(db, &id)?;
    println!("{content}");

    Ok(())
}
