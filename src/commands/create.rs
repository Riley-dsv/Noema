use std::path::PathBuf;

use crate::{database::sqlite, editor::open_editor, error::NoemaResult};

pub fn create_note(db: Option<PathBuf>, title: String, content: Option<String>) -> NoemaResult {
    let editor_content = open_editor(&content.unwrap_or_default())?;
    sqlite::insert(db, title, editor_content)?;
    Ok(())
}
