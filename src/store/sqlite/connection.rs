use super::SQLStore;
use rusqlite::Connection;

impl SQLStore {
    pub fn open(path: impl AsRef<std::path::Path>) -> crate::error::NoemaResult<Self> {
        let connection = Connection::open(path)?;

        connection.execute("PRAGMA foreign_keys = ON", [])?;

        let store = Self { connection };
        Ok(store)
    }
}
