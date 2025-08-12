-- Add migration script here
-- Rename original table --
ALTER TABLE audio_books RENAME TO audio_books_old;

-- Create new table with updated schema --
CREATE TABLE IF NOT EXISTS audio_books (
    identifier TEXT PRIMARY KEY NOT NULL,
    title      TEXT,
    author     TEXT,
    created_at TEXT,
    updated_at TEXT,
    is_loved   BOOLEAN DEFAULT 0
);

-- Copy data over --
INSERT INTO audio_books (identifier, title, created_at, updated_at, is_loved)
SELECT identifier, title, created_at, updated_at, is_loved FROM audio_books_old;

-- Drop old table --
DROP TABLE audio_books_old;