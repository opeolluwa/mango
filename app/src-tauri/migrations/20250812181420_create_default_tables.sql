-- Add migration script here
-- AUDIO BOOKS TABLE --
CREATE TABLE
    IF NOT EXISTS audio_books (
        identifier TEXT PRIMARY KEY NOT NULL,
        title TEXT,
        created_at TEXT,
        updated_at TEXT,
        is_loved TINYINT DEFAULT 0
    );

-- PLAYLIST TABLE --
CREATE TABLE
    IF NOT EXISTS playlist (
        identifier TEXT PRIMARY KEY NOT NULL,
        name TEXT,
        description TEXT
    );

-- HISTORY TABLE --
CREATE TABLE
    IF NOT EXISTS history (
        identifier TEXT PRIMARY KEY NOT NULL,
        audio_book_identifier TEXT,
        FOREIGN KEY (audio_book_identifier) REFERENCES audio_books (identifier) ON DELETE CASCADE
    );

-- -- VERIFY TABLES CREATED --
-- SELECT
--     name
-- FROM
--     sqlite_master
-- WHERE
--     type = 'table';