use std::path::PathBuf;

use chrono;
use rusqlite::{Connection, Result, Row, params};
use uuid::Uuid;

use crate::database::migration::{CURRENT_MIGRATION_VERSION, INIT_SCHEMA, MIGRATIONS};

#[derive(Debug)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct NoteSummary {
    pub id: String,
    pub title: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub struct TagSummary {
    pub name: String,
    pub total_attached: i64,
}

pub struct SQLStore {
    connection: Connection,
}

impl SQLStore {
    #[allow(dead_code)]
    fn execute_batch_debug(conn: &rusqlite::Connection, sql: &str) -> rusqlite::Result<()> {
        for statement in sql.split(';') {
            let statement = statement.trim();

            if statement.is_empty() {
                continue;
            }

            eprintln!("\nSQL => {}\n", statement);

            conn.execute_batch(statement).map_err(|err| {
                eprintln!("FAILED SQL => {}\nERROR => {}", statement, err);
                err
            })?;
        }

        Ok(())
    }

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
        self.connection.execute_batch(INIT_SCHEMA)?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<()> {
        if self.table_exists("notes")? {
            let id_type = self.get_id_field_type();
            if matches!(id_type, Ok(Some(field_type)) if field_type == "INTEGER") {
                self.normalize_notes_integer_id_to_text()?;
            }
        }

        let applied = self.applied_migration()?;
        let current_version = applied.last().copied().unwrap_or(0);

        if current_version < CURRENT_MIGRATION_VERSION {
            for migration in MIGRATIONS {
                if migration.version > current_version {
                    self.connection.execute_batch(migration.sql)?;
                    self.update_migration_count(&migration.version)?;
                }
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

    pub fn insert_tag(&self, tag_name: &str) -> Result<()> {
        self.connection
            .execute("INSERT INTO tags (name) VALUES (?1)", params![tag_name])?;

        Ok(())
    }

    pub fn update_note_tags(&self, note_id: &str, tag_id: &i32) -> Result<()> {
        self.connection.execute(
            "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            params![note_id, tag_id],
        )?;

        Ok(())
    }

    pub fn delete_tag_from_note(&self, note_id: &str, tag_id: &i32) -> Result<usize> {
        let mut statement = self
            .connection
            .prepare("DELETE FROM note_tags WHERE note_id = ?1 AND tag_id = ?2")?;

        let deleted = statement.execute(params![note_id, tag_id])?;

        Ok(deleted)
    }

    pub fn delete_tag(&self, tag_id: &i32) -> Result<usize> {
        let mut statement = self.connection.prepare("DELETE FROM tags WHERE id=?1")?;

        let deleted = statement.execute(params![tag_id])?;

        Ok(deleted)
    }

    pub fn list_tags(&self) -> Result<Vec<TagSummary>> {
        let mut statement = self.connection.prepare(
            "
              SELECT tags.name, COUNT(note_tags.note_id) AS total_attached 
              FROM tags 
              LEFT JOIN note_tags ON note_tags.tag_id = tags.id 
              GROUP BY tags.id, tags.name 
              ORDER BY tags.name
            ",
        )?;

        statement
            .query_map([], tag_summary_from_row)?
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn tag_exists(&self, tag_name: &str) -> Result<bool> {
        self.connection.query_row(
            "SELECT 1  FROM tags WHERE name = ?1",
            params![tag_name],
            |row| row.get::<_, bool>(0),
        )
    }

    pub fn get_id_from_tag_name(&self, tag_name: &str) -> Result<i32> {
        self.connection.query_row(
            "SELECT id FROM tags WHERE name = ?1",
            params![tag_name],
            |row| row.get(0),
        )
    }

    pub fn filter_notes_by_tag(&self, tag_id: &i32) -> Result<Vec<NoteSummary>> {
        let mut statement = self.connection.prepare(
            "
              SELECT id, title, updated_at 
              FROM notes 
              LEFT JOIN note_tags ON note_tags.note_id = notes.id
              WHERE note_tags.tag_id = ?1
            ",
        )?;

        statement
            .query_map([tag_id], summary_from_row)?
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn filter_tags_by_note(&self, note_id: &str) -> Result<Vec<String>> {
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

    fn applied_migration(&self) -> Result<Vec<i32>> {
        let mut statement = self
            .connection
            .prepare("SELECT version FROM schema_migrations ORDER BY version ASC")?;

        let versions = statement
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<i32>, rusqlite::Error>>()?;

        Ok(versions)
    }

    fn normalize_notes_integer_id_to_text(&self) -> Result<()> {
        self.connection.execute_batch(
            "
          PRAGMA foreign_keys = OFF;
          BEGIN TRANSACTION;

          ALTER TABLE notes RENAME TO notes_old;

          CREATE TABLE notes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT,
            created_at TEXT,
            updated_at TEXT
          );

          INSERT INTO notes (id, title, content, created_at, updated_at)
          SELECT CAST(id as TEXT), title, content, created_at, updated_at
          FROM notes_old;

          DROP TABLE notes_old;

          COMMIT;

          PRAGMA foreign_keys = ON;
          ",
        )?;

        Ok(())
    }

    fn get_id_field_type(&self) -> Result<Option<String>> {
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

    fn table_exists(&self, table_name: &str) -> Result<bool> {
        self.connection.query_row(
            "SELECT EXISTS (
              SELECT 1
              FROM sqlite_master
              WHERE type = 'table'
                AND name = ?1
          );",
            params![table_name],
            |row| row.get(0),
        )
    }

    fn update_migration_count(&self, migration_version: &i32) -> Result<()> {
        let now = chrono::offset::Local::now().to_rfc3339();
        self.connection.execute(
            "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
            params![migration_version, now],
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

fn summary_from_row(row: &Row) -> Result<NoteSummary> {
    Ok(NoteSummary {
        id: row.get("id")?,
        title: row.get("title")?,
        updated_at: row.get("updated_at")?,
    })
}

fn tag_summary_from_row(row: &Row) -> Result<TagSummary> {
    Ok(TagSummary {
        name: row.get("name")?,
        total_attached: row.get::<_, i64>("total_attached")?,
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
