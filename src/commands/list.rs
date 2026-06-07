use std::path::PathBuf;

use rusqlite::Result;

use crate::database::sqlite;

pub fn list_notes(db: Option<PathBuf>) -> Result<()> {
    sqlite::list(db)?;
    Ok(())
}
