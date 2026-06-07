use std::path::PathBuf;

use rusqlite::Result;

use crate::{database::sqlite, editor::open_editor};

pub fn update_note(db: Option<PathBuf>, id: &str, title: Option<String>) -> Result<()> {
    if let Some(title) = title {
        sqlite::update_title(db.clone(), &id, title)?;
    } else {
        let old_content = sqlite::get_content(db.clone(), &id)?;
        let new_content = open_editor(&old_content);
        if let Ok(new_content) = new_content {
            sqlite::update(db, &new_content, id.to_string())?;
        }
    }

    Ok(())
}
