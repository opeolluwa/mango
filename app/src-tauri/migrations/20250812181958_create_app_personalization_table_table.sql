-- Add migration script here
-- Create a new table that stores app personalization --
CREATE TABLE
    IF NOT EXISTS app_personalization (
        theme TEXT DEFAULT 'light',
        language TEXT,
        first_name TEXT,
        last_name TEXT,
        email TEXT,
        preferred_voice TEXT
    );