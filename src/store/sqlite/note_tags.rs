use rusqlite::{Result, params};

use crate::store::sqlite::SQLStore;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct NoteSummary {
    pub id: String,
    pub title: String,
    pub updated_at: String,
}

pub trait NoteTagsStore {
    fn search_content(&self, keyword: &str) -> Result<Vec<NoteSummary>>;
    fn update_note_tags(&self, note_id: &str, tag_id: &i32) -> Result<()>;
    fn delete_tag_from_note(&self, note_id: &str, tag_id: &i32) -> Result<usize>;
    fn filter_notes_by_tag(&self, tag_id: &i32) -> Result<Vec<NoteSummary>>;
    fn filter_tags_by_note(&self, note_id: &str) -> Result<Vec<String>>;
}

impl NoteTagsStore for SQLStore {
    fn search_content(&self, keyword: &str) -> Result<Vec<NoteSummary>> {
        let pattern = format!("%{}%", keyword);

        let mut statement = self.connection.prepare("SELECT id, title, updated_at FROM notes WHERE content LIKE ?1 OR title LIKE ?1 ORDER BY updated_at DESC")?;

        Ok(statement
            .query_map(params![pattern], SQLStore::summary_from_row)?
            .collect::<Result<Vec<_>>>()?)
    }

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

    fn filter_notes_by_tag(&self, tag_id: &i32) -> Result<Vec<NoteSummary>> {
        let mut statement = self.connection.prepare(
            "
              SELECT id, title, updated_at 
              FROM notes 
              LEFT JOIN note_tags ON note_tags.note_id = notes.id
              WHERE note_tags.tag_id = ?1
            ",
        )?;

        statement
            .query_map([tag_id], SQLStore::summary_from_row)?
            .collect::<Result<Vec<_>, _>>()
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
