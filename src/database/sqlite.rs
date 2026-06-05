use std::path::PathBuf;

use chrono;
use rusqlite::{Connection, Result, params};

use crate::database::path;

fn connect(db_path: Option<PathBuf>) -> Result<Connection> {
    let path = db_path
        .map(PathBuf::from)
        .unwrap_or_else(path::default_database_path);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("failed to create database directory");
    }

    Connection::open(path)
}

#[derive(Debug)]
struct Note {
    id: i32,
    title: String,
    content: String,
    created_at: String,
    updated_at: String,
}

pub fn init(db_path: Option<PathBuf>) -> Result<()> {
    let conn = connect(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      title TEXT NOT NULL,
      content TEXT,
      created_at TEXT,
      updated_at TEXT
    )",
        (),
    )?;

    Ok(())
}

pub fn insert(db_path: Option<PathBuf>, note_title: String, note_content: String) -> Result<()> {
    let conn = connect(db_path)?;
    let now = chrono::offset::Local::now().to_rfc3339();

    conn.execute(
        "INSERT INTO notes (title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        params![note_title, note_content, now, now],
    )?;

    Ok(())
}

pub fn list(db_path: Option<PathBuf>) -> Result<()> {
    let conn = connect(db_path)?;
    let mut statement = conn.prepare(
        "SELECT id, title, content, created_at, updated_at FROM notes ORDER BY updated_at DESC",
    )?;

    let note_iter = statement.query_map([], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;

    for note_result in note_iter {
        let note = note_result?;
        println!(
            "Found note: {} | {} | {}",
            note.id, note.title, note.updated_at
        )
    }

    Ok(())
}

pub fn delete(db_path: Option<PathBuf>, id: String) -> Result<()> {
    let conn = connect(db_path)?;

    conn.execute("DELETE FROM notes WHERE id=?1", params![id])?;

    Ok(())
}

pub fn get_content(db_path: Option<PathBuf>, id: &str) -> Result<String> {
    let conn = connect(db_path)?;
    let content = conn.query_row(
        "SELECT content FROM notes WHERE id=?1",
        params![id],
        |row| row.get(0),
    );

    Ok(content?)
}

pub fn select(db_path: Option<PathBuf>, id: String) -> Result<()> {
    let conn = connect(db_path)?;
    let note = conn.query_row(
        "SELECT id, title, content, created_at, updated_at FROM notes WHERE id=?1",
        params![id],
        |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        },
    )?;

    println!("{note:#?}");

    Ok(())
}

pub fn update(db_path: Option<PathBuf>, new_content: &str, id: String) -> Result<()> {
    let conn = connect(db_path)?;
    let now = chrono::offset::Local::now().to_rfc3339();

    println!("{id}");

    conn.execute(
        "UPDATE notes SET content=?1, updated_at=?2 WHERE id=?3",
        params![new_content, now, id],
    )?;

    Ok(())
}

pub fn update_title(db_path: Option<PathBuf>, id: &str, title: String) -> Result<()> {
    let conn = connect(db_path)?;
    let now = chrono::offset::Local::now().to_rfc3339();

    conn.execute(
        "UPDATE notes SET title=?1 updated_at=?2 WHERE id=?3",
        params![title, now, id],
    )?;

    Ok(())
}
