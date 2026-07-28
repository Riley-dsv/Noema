use rusqlite::{Result, params};

use crate::store::sqlite::SQLStore;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct NoteSummary {
    pub id: String,
    pub title: String,
    pub updated_at: String,
}

pub trait NoteTagsStore {
    fn update_note_tags(&self, note_id: &str, tag_id: &i32) -> Result<()>;
    fn delete_tag_from_note(&self, note_id: &str, tag_id: &i32) -> Result<usize>;
    fn filter_tags_by_note(&self, note_id: &str) -> Result<Vec<String>>;
}

impl NoteTagsStore for SQLStore {
    fn update_note_tags(&self, note_id: &str, tag_id: &i32) -> Result<()> {
        self.connection.execute(
            "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            params![note_id, tag_id],
        )?;

        Ok(())
    }

    fn delete_tag_from_note(&self, note_id: &str, tag_id: &i32) -> Result<usize> {
        let mut statement = self
            .connection
            .prepare("DELETE FROM note_tags WHERE note_id = ?1 AND tag_id = ?2")?;

        let deleted = statement.execute(params![note_id, tag_id])?;

        Ok(deleted)
    }

    fn filter_tags_by_note(&self, note_id: &str) -> Result<Vec<String>> {
        let mut statement = self.connection.prepare(
            "
            SELECT name 
            FROM tags 
            LEFT JOIN note_tags ON note_tags.tag_id = tags.id 
            WHERE note_tags.note_id = ?1
          ",
        )?;

        statement
            .query_map([note_id], |row| row.get(0))?
            .collect::<Result<_>>()
    }
}
