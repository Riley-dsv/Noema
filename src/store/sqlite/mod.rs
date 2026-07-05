pub mod connection;
pub mod migration;
pub mod notes;
pub mod row;
pub mod tags;

use rusqlite::Connection;

pub struct SQLStore {
    pub(crate) connection: Connection,
}
