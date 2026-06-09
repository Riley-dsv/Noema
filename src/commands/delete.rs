use std::{io, path::PathBuf};

use crate::{database::sqlite, error::NoemaResult};

pub fn delete_note(db: Option<PathBuf>, id: &str) -> NoemaResult {
    print!("Are you sure you want to delete note: {} (Y/N) > ", id);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read stdin");

    if input.trim().to_lowercase() == "y" {
        sqlite::delete(db, id.to_string())?;
    }

    Ok(())
}
