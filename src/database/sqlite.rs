use std::path::PathBuf;

use rusqlite::{Connection, Result, Row, params};

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

    // Used in tests so not so dead.
    #[allow(dead_code)]
    pub fn open_in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        Ok(Self { connection })
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
}

fn connect(path: PathBuf) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("failed to create database directory");
    }

    Connection::open(path)
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
