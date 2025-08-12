-- Add migration script here
-- Add timestamp columns to playlist table --
ALTER TABLE playlist ADD COLUMN created_at TEXT;
ALTER TABLE playlist ADD COLUMN updated_at TEXT;

-- Update existing rows with current timestamp --
UPDATE playlist
SET created_at = CURRENT_TIMESTAMP,
    updated_at = CURRENT_TIMESTAMP
WHERE created_at IS NULL;

-- Make the columns NOT NULL for future inserts --
PRAGMA foreign_keys=off;

CREATE TABLE playlist_new (
    identifier  TEXT PRIMARY KEY NOT NULL,
    name        TEXT,
    description TEXT,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

INSERT INTO playlist_new SELECT * FROM playlist;
DROP TABLE playlist;
ALTER TABLE playlist_new RENAME TO playlist;

PRAGMA foreign_keys=on;