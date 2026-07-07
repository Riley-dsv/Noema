use rusqlite::{Result, params};
use uuid::Uuid;

use crate::store::sqlite::SQLStore;

#[derive(Debug)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

fn new_note_id() -> String {
    Uuid::new_v4()
        .simple()
        .to_string()
        .chars()
        .take(8)
        .collect()
}

pub trait NoteStore {
    fn insert_note(&self, note_title: &str, note_content: &str) -> Result<String>;
    fn list_notes(&self) -> Result<Vec<Note>>;
    fn delete_note(&self, note_id: &str) -> Result<usize>;
    fn get_note(&self, note_id: &str) -> Result<Note>;
    fn get_content(&self, note_id: &str) -> Result<String>;
    fn update_content(&self, note_id: &str, new_content: &str) -> Result<()>;
    fn update_title(&self, note_id: &str, new_title: &str) -> Result<()>;
}

impl NoteStore for SQLStore {
    fn insert_note(&self, note_title: &str, note_content: &str) -> Result<String> {
        let id = new_note_id();
        let now = chrono::offset::Local::now().to_rfc3339();

        self.connection.execute(
            "
              INSERT INTO notes (id, title, content. created_at, updated_at) 
              VALUES (?1, ?2, ?3, ?4, ?5)
            ",
            params![id, note_title, note_content, now, now],
        )?;

        Ok(id)
    }

    fn get_note(&self, note_id: &str) -> Result<Note> {
        self.connection.query_row(
            "SELECT id, title, content, created_at, updated_at FROM notes WHERE id=?1",
            params![note_id],
            SQLStore::note_from_row,
        )
    }

    fn get_content(&self, note_id: &str) -> Result<String> {
        Ok(self.get_note(note_id)?.content)
    }

    fn update_title(&self, note_id: &str, new_title: &str) -> Result<()> {
        let now = chrono::offset::Local::now().to_rfc3339();
        self.connection.execute(
            "UPDATE notes SET title=?1, updated_at=?2 WHERE id=?3",
            params![new_title, now, note_id],
        )?;

        Ok(())
    }

    fn update_content(&self, note_id: &str, new_content: &str) -> Result<()> {
        let now = chrono::offset::Local::now().to_rfc3339();
        self.connection.execute(
            "UPDATE notes SET content=?1, updated_at=?2 WHERE id=?3",
            params![new_content, now, note_id],
        )?;

        Ok(())
    }

    fn list_notes(&self) -> Result<Vec<Note>> {
        let mut statement = self.connection.prepare(
            "SELECT id, title, content, created_at, updated_at FROM notes ORDER BY updated_at DESC",
        )?;

        Ok(statement
            .query_map([], SQLStore::note_from_row)?
            .collect::<Result<Vec<_>>>()?)
    }

    fn delete_note(&self, note_id: &str) -> Result<usize> {
        let mut statement = self.connection.prepare("DELETE FROM notes WHERE id=?1")?;

        let deleted = statement.execute(params![note_id])?;

        Ok(deleted)
    }
}
