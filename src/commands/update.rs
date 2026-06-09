use std::path::PathBuf;

use crate::{database::sqlite, editor::open_editor, error::NoemaResult};

pub fn update_note(db: Option<PathBuf>, id: &str, title: Option<String>) -> NoemaResult {
    if let Some(title) = title {
        sqlite::update_title(db.clone(), &id, title)?;
    } else {
        let old_content = sqlite::get_content(db.clone(), &id)?;
        let new_content = open_editor(&old_content)?;
        sqlite::update(db, &new_content, id.to_string())?;
    }

    Ok(())
}
