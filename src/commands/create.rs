use std::path::PathBuf;

use rusqlite::Result;

use crate::{database::sqlite, editor::open_editor};

pub fn create_note(db: Option<PathBuf>, title: String, content: Option<String>) -> Result<()> {
    let editor_content = open_editor(&content.unwrap_or_default());
    if let Ok(editor_content) = editor_content {
        sqlite::insert(db, title, editor_content)?;
    }

    Ok(())
}
