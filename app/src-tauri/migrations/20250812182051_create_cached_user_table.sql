-- Add migration script here
-- Create a new table that stores cached user information --
CREATE TABLE IF NOT EXISTS cached_user (
    identifier TEXT PRIMARY KEY NOT NULL,
    first_name TEXT,
    last_name  TEXT,
    email      TEXT,
    avatar_url TEXT
);