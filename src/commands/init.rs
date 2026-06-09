use std::path::PathBuf;

use crate::{database::sqlite, error::NoemaResult};

pub fn init_database(path: Option<PathBuf>) -> NoemaResult {
    sqlite::init(path)?;
    Ok(())
}
