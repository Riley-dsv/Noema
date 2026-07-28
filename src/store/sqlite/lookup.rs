use rusqlite::{Result, params};

use crate::store::sqlite::{SQLStore, note_tags::NoteSummary};

pub trait LookupStore {
    fn search_content(&self, keyword: &str) -> Result<Vec<NoteSummary>>;
    fn search_title(&self, keyword: &str) -> Result<Vec<NoteSummary>>;
    fn search_tags(&self, tags: Vec<String>) -> Result<Vec<NoteSummary>>;
    fn search_note_format(&self, format: &str) -> Result<Vec<NoteSummary>>;
    fn filter_notes_by_tag(&self, tag_id: &i32) -> Result<Vec<NoteSummary>>;
}

impl LookupStore for SQLStore {
    fn search_content(&self, keyword: &str) -> Result<Vec<NoteSummary>> {
        let pattern = format!("%{}%", keyword);

        let mut statement = self.connection.prepare("SELECT id, title, updated_at FROM notes WHERE content LIKE ?1 ORDER BY updated_at DESC")?;

        Ok(statement
            .query_map(params![pattern], SQLStore::summary_from_row)?
            .collect::<Result<Vec<_>>>()?)
    }

    fn search_title(&self, keyword: &str) -> Result<Vec<NoteSummary>> {
        let pattern = format!("%{}%", keyword);

        let mut statement = self.connection.prepare(
            "SELECT id, title, updated_at FROM notes WHERE title LIKE ?1 ORDER BY updated_at DESC",
        )?;

        Ok(statement
            .query_map(params![pattern], SQLStore::summary_from_row)?
            .collect::<Result<Vec<_>>>()?)
    }

    fn search_tags(&self, _tags: Vec<String>) -> Result<Vec<NoteSummary>> {
        unimplemented!();
    }

    fn search_note_format(&self, _format: &str) -> Result<Vec<NoteSummary>> {
        unimplemented!();
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
}
