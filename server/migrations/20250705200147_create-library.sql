-- Add migration script here
CREATE TABLE
    IF NOT EXISTS audio_books (
        identifer UUID PRIMARY KEY,
        user_identifier UUID NOT NULL REFERENCES users (identifier),
        audio_source VARCHAR NOT NULL,
        file_name VARCHAR NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ DEFAULT NULL,
        last_played TIMESTAMPTZ DEFAULT NULL
    )