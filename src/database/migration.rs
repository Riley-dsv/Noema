pub struct Migration {
    pub version: i32,
    pub sql: &'static str,
}

pub const CURRENT_MIGRATION_VERSION: i32 = 2;

pub const INIT_SCHEMA: &str = "

    BEGIN TRANSACTION;
    
    CREATE TABLE IF NOT EXISTS schema_migrations (
        version INTEGER PRIMARY KEY DEFAULT 1,
        applied_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS notes (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        content TEXT,
        created_at TEXT,
        updated_at TEXT
    );

    CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE
    );

    CREATE TABLE IF NOT EXISTS note_tags (
        note_id TEXT NOT NULL,
        tag_id INTEGER NOT NULL,

        PRIMARY KEY (note_id, tag_id),

        FOREIGN KEY (note_id)
            REFERENCES notes(id)
            ON DELETE CASCADE

        FOREIGN KEY (tag_id)
            REFERENCES tags(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_note_tags
        ON note_tags(tag_id);

    COMMIT;
";

pub const MIGRATIONS: &[Migration] = &[Migration {
    version: 2,
    sql: "

          BEGIN TRANSACTION;

          CREATE TABLE IF NOT EXISTS schema_migrations (
              version INTEGER PRIMARY KEY,
              applied_at TEXT NOT NULL
          );

          CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
          );

          CREATE TABLE IF NOT EXISTS note_tags (
            note_id TEXT NOT NULL,
            tag_id INTEGER NOT NULL,

            PRIMARY KEY (note_id, tag_id),

            FOREIGN KEY (note_id)
              REFERENCES notes(id)
              ON DELETE CASCADE

            FOREIGN KEY (tag_id)
              REFERENCES tags(id)
              ON DELETE CASCADE
          );

          CREATE INDEX IF NOT EXISTS idx_note_tags
            ON note_tags(tag_id);

          COMMIT;
        ",
}];
