-- Add migration script here
ALTER TABLE albulm
ADD COLUMN is_loved BOOLEAN DEFAULT false;

ALTER TABLE albulm
ADD COLUMN playlist_identifier UUID REFERENCES playlist (identifier);