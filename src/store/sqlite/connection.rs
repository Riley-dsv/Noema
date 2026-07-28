use super::SQLStore;
use rusqlite::Connection;

impl SQLStore {
    pub fn open(path: std::path::PathBuf) -> crate::error::NoemaResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("failed to create database directory");
        }

        let connection = Connection::open(path)?;

        connection.execute("PRAGMA foreign_keys = ON", [])?;

        let store = Self { connection };
        Ok(store)
    }

    #[allow(dead_code)]
    // Used in test, not real dead code
    pub fn open_in_memory() -> crate::error::NoemaResult<Self> {
        let connection = Connection::open_in_memory()?;
        connection.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(Self { connection })
    }
}
