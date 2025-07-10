-- Add migration script here
ALTER TABLE audio_books
ADD COLUMN is_loved BOOLEAN DEFAULT false;

ALTER TABLE audio_books
ADD COLUMN playlist_identifier UUID REFERENCES playlist (identifier);