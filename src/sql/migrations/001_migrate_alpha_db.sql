ALTER TABLE notes RENAME TO notes_old;

CREATE TABLE notes (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT,
    created_at TEXT,
    updated_at TEXT
);

INSERT INTO notes (id, title, content, created_at, updated_at)
SELECT
    CAST(id AS TEXT) AS id,
    title,
    content,
    created_at,
    updated_at
FROM notes_old;

DROP TABLE notes_old;
