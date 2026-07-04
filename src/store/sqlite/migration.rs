use rusqlite::{Result, params};

use super::SQLStore;

struct Migration {
    version: i32,
    sql: &'static str,
}

const CURRENT_MIGRATION_VERSION: i32 = 2;

const MIGRATIONS: &[Migration] = &[Migration {
    version: 2,
    sql: include_str!("../../sql/migrations/002_tags.sql"),
}];

impl SQLStore {
    pub fn init(&mut self) -> Result<()> {
        let transaction = self.connection.transaction()?;

        transaction.execute_batch(include_str!("../../sql/init.sql"))?;

        transaction.commit()?;

        Ok(())
    }

    pub fn migrate(&mut self) -> Result<()> {
        if self.table_exists("notes")? {
            let id_type = self.get_id_field_type();
            if matches!(id_type, Ok(Some(field_type)) if field_type == "INTEGER") {
                self.migrate_alpha_database()?;
            }
        }

        let applied = self.applied_migration()?;
        let current_version = applied.last().copied().unwrap_or(0);

        if current_version < CURRENT_MIGRATION_VERSION {
            for migration in MIGRATIONS {
                if migration.version > current_version {
                    let transaction = self.connection.transaction()?;
                    transaction.execute_batch(migration.sql)?;
                    transaction.commit()?;
                    self.update_migration_count(&migration.version)?;
                }
            }
        }

        Ok(())
    }

    fn migrate_alpha_database(&mut self) -> Result<()> {
        let transaction = self.connection.transaction()?;
        transaction.execute_batch(include_str!(
            "../../sql/migrations/001_migrate_alpha_db.sql"
        ))?;
        transaction.commit()?;

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

    fn update_migration_count(&self, migration_version: &i32) -> Result<()> {
        let now = chrono::offset::Local::now().to_rfc3339();
        self.connection.execute(
            "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
            params![migration_version, now],
        )?;

        Ok(())
    }
}
