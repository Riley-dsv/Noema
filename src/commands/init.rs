use std::path::PathBuf;

use rusqlite::Result;

use crate::database::sqlite;

pub fn init_database(path: Option<PathBuf>) -> Result<()> {
    sqlite::init(path)?;
    Ok(())
}
