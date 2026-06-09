use std::path::PathBuf;

use crate::{database::sqlite, error::NoemaResult};

pub fn list_notes(db: Option<PathBuf>) -> NoemaResult {
    sqlite::list(db)?;
    Ok(())
}
