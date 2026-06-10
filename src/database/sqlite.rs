use std::path::PathBuf;

use chrono;
use rusqlite::{Connection, Result, Row, params};
use uuid::Uuid;

use crate::database::migration::MIGRATIONS;

#[derive(Debug)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub struct NoteSummary {
    pub id: String,
    pub title: String,
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

    // Used in tests so not so dead.
    #[allow(dead_code)]
    pub fn open_in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        Ok(Self { connection })
    }

    pub fn init(&self) -> Result<()> {
        self.migrate()?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
              version INTEGER PRIMARY KEY,
              applied_at TEXT NOT NULL
            )",
            [],
        )?;

        let applied = self.applied_migration()?;
        let now = chrono::offset::Local::now().to_rfc3339();

        for migration in MIGRATIONS {
            if !applied.contains(&migration.version) {
                self.connection.execute_batch(migration.sql)?;

                self.connection.execute(
                    "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
                    params![migration.version, now],
                )?;
            }
        }

        Ok(())
    }

    pub fn insert_note(&self, note_title: &str, note_content: &str) -> Result<String> {
        let id = new_note_id();
        let now = chrono::offset::Local::now().to_rfc3339();

        self.connection.execute(
            "INSERT INTO notes (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, note_title, note_content, now, now],
        )?;

        Ok(id)
    }

    pub fn search_content(&self, keyword: &str) -> Result<Vec<NoteSummary>> {
        let pattern = format!("%{}%", keyword);

        let mut statement = self.connection.prepare(
          "SELECT id, title, updated_at FROM notes WHERE content LIKE ?1 OR title LIKE ?1 ORDER BY updated_at DESC"
        )?;

        Ok(statement
            .query_map([pattern], summary_from_row)?
            .collect::<Result<Vec<_>>>()?)
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
        let mut statement = self.connection.prepare("DELETE FROM notes WHERE id=?1")?;

        let deleted = statement.execute(params![id])?;

        if deleted == 0 {
            println!("No note found with id {id}");
        }

        Ok(deleted)
    }

    pub fn get_content(&self, id: &str) -> Result<String> {
        let content = self.connection.query_row(
            "SELECT content FROM notes WHERE id=?1",
            params![&id],
            |note| note.get("content"),
        )?;

        Ok(content)
    }

    pub fn get_note(&self, id: &str) -> Result<Note> {
        let note = self.connection.query_row(
            "SELECT id, title, content, created_at, updated_at FROM notes WHERE id=?1",
            params![&id],
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

    fn applied_migration(&self) -> Result<Vec<i32>> {
        let mut statement = self
            .connection
            .prepare("SELECT version FROM schema_migrations ORDER BY version ASC")?;

        let versions = statement
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<i32>, rusqlite::Error>>()?;

        Ok(versions)
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

fn summary_from_row(row: &Row) -> Result<NoteSummary> {
    Ok(NoteSummary {
        id: row.get("id")?,
        title: row.get("title")?,
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
