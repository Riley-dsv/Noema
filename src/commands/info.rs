use std::path::PathBuf;

use rusqlite::Result;

use crate::database::sqlite;

pub fn note_info(db: Option<PathBuf>, id: &str) -> Result<()> {
    let note = sqlite::select(db.clone(), id.to_string())?;
    let content_size = String::from(sqlite::get_content(db, &id)?).len();
    println!(
        "ID: {}\nTitle: {}\nCreated At: {}\nLast Updated: {}\nSize: {}Bytes",
        &id, note.title, note.created_at, note.updated_at, content_size
    );

    Ok(())
}
