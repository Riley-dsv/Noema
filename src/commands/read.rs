use std::path::PathBuf;

use rusqlite::Result;

use crate::database::sqlite;

pub fn read_note(db: Option<PathBuf>, id: &str) -> Result<()> {
    let content = sqlite::get_content(db, &id)?;
    println!("{content}");

    Ok(())
}
