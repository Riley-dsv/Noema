use std::path::PathBuf;

use chrono;
use rusqlite::{Connection, Result, Row, params};
use uuid::Uuid;

#[derive(Debug)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct SQLStore {
    connection: Connection,
}

impl SQLStore {
    pub fn open(db_path: PathBuf) -> Result<Self> {
        let connection = connect(db_path)?;
        Ok(Self { connection })
    }

    pub fn init(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS notes (
              id TEXT PRIMARY KEY,
              title TEXT NOT NULL,
              content TEXT,
              created_at TEXT,
              updated_at TEXT
            )",
            (),
        )?;

        Ok(())
    }

    pub fn insert_note(&self, note_title: &str, note_content: &str) -> Result<()> {
        let id = new_note_id();
        let now = chrono::offset::Local::now().to_rfc3339();

        self.connection.execute(
            "INSERT INTO notes (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![id, note_title, note_content, now, now],
        )?;

        Ok(())
    }

    pub fn list_notes(&self) -> Result<Vec<Note>> {
        let mut statement = self.connection.prepare(
            "SELECT id, title, content, created_at, updated_at FROM notes ORDER BY updated_at DESC",
        )?;

        Ok(statement
            .query_map([], note_from_row)?
            .collect::<Result<Vec<_>>>()?)
    }

    pub fn delete_note(&self, id: &str) -> Result<usize> {
        let deleted = self
            .connection
            .execute("DELETE FROM notes WHERE id=?1", params![id])?;

        if deleted == 0 {
            print!("No note found with id {id}");
        }

        Ok(deleted)
    }

    pub fn get_content(&self, id: &str) -> Result<String> {
        let note = self.connection.query_row(
            "SELECT content FROM notes WHERE id=?1",
            params![id],
            note_from_row,
        );

        Ok(note.unwrap().content)
    }

    pub fn get_note(&self, id: &str) -> Result<Note> {
        let note = self.connection.query_row(
            "SELECT id, title, content, created_at, updated_at FROM notes WHERE id=?1",
            params![id],
            note_from_row,
        )?;

        Ok(note)
    }
    pub fn update_content(&self, id: &str, new_content: &str) -> Result<()> {
        let now = chrono::offset::Local::now().to_rfc3339();

        self.connection.execute(
            "UPDATE notes SET content=?1, updated_at=?2 WHERE id=?3",
            params![new_content, now, id],
        )?;

        Ok(())
    }

    pub fn update_title(&self, id: &str, title: &str) -> Result<()> {
        let now = chrono::offset::Local::now().to_rfc3339();

        self.connection.execute(
            "UPDATE notes SET title=?1, updated_at=?2 WHERE id=?3",
            params![title, now, id],
        )?;

        Ok(())
    }
}

fn connect(path: PathBuf) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("failed to create database directory");
    }

    Connection::open(path)
}

fn note_from_row(row: &Row) -> Result<Note> {
    Ok(Note {
        id: row.get("id")?,
        title: row.get("title")?,
        content: row.get("content")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

fn new_note_id() -> String {
    Uuid::new_v4()
        .simple()
        .to_string()
        .chars()
        .take(8)
        .collect()
}
