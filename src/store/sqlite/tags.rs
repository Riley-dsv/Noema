use rusqlite::{Result, params};

use crate::store::sqlite::SQLStore;

#[derive(Debug)]
pub struct TagSummary {
    pub name: String,
    pub total_attached: i64,
}

trait TagsStore {
    fn insert_tag(&self, tag_name: &str) -> Result<()>;
    fn delete_tag(&self, tag_id: &i32) -> Result<usize>;
    fn list_tags(&self) -> Result<Vec<TagSummary>>;
    fn tag_exists(&self, tag_name: &str) -> Result<bool>;
    fn get_id_from_tag_name(&self, tag_name: &str) -> Result<i32>;
}

impl TagsStore for SQLStore {
    fn insert_tag(&self, tag_name: &str) -> Result<()> {
        self.connection
            .execute("INSERT INTO tags (name) VALUES (?1)", params![tag_name])?;

        Ok(())
    }

    fn delete_tag(&self, tag_id: &i32) -> Result<usize> {
        let mut statement = self.connection.prepare("DELETE FROM tags WHERE id=?1")?;

        let deleted = statement.execute(params![tag_id])?;

        Ok(deleted)
    }

    fn list_tags(&self) -> Result<Vec<TagSummary>> {
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
            .query_map([], self.tag_summary_from_row)?
            .collect::<Result<Vec<_>, _>>()
    }

    fn tag_exists(&self, tag_name: &str) -> Result<bool> {
        self.connection.query_row(
            "SELECT 1  FROM tags WHERE name = ?1",
            params![tag_name],
            |row| row.get::<_, bool>(0),
        )
    }

    fn get_id_from_tag_name(&self, tag_name: &str) -> Result<i32> {
        self.connection.query_row(
            "SELECT id FROM tags WHERE name = ?1",
            params![tag_name],
            |row| row.get(0),
        )
    }
}
