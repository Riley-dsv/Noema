use rusqlite::{Result, Row, params};

use crate::store::sqlite::{SQLStore, notes::Note, tags::TagSummary};

impl SQLStore {
    pub fn table_exists(&self, table_name: &str) -> Result<bool> {
        self.connection.query_row(
            "
            SELECT EXISTS ( 
              SELECT 1 
              FROM sqlite_master 
              WHERE type = 'table' 
                AND name = ?1
            )",
            params![table_name],
            |row| row.get(0),
        )
    }

    pub fn get_id_field_type(&self) -> Result<Option<String>> {
        let mut statement = self.connection.prepare("PRAGMA table_info(notes)")?;

        let rows = statement.query_map([], |row| {
            Ok((row.get::<_, String>(1)?, row.get::<_, String>(2)?))
        })?;

        for row in rows {
            let (name, rtype) = row?;
            if name == "id" {
                return Ok(Some(rtype));
            }
        }

        Ok(None)
    }

    pub fn note_from_row(row: &Row) -> Result<Note> {
        Ok(Note {
            id: row.get("id")?,
            title: row.get("title")?,
            content: row.get("content")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }

    pub fn tag_summary_from_row(row: &Row) -> Result<TagSummary> {
        Ok(TagSummary {
            name: row.get("name")?,
            total_attached: row.get::<_, i64>("total_attached")?,
        })
    }
}
