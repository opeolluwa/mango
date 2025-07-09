-- Add migration script here
ALTER TABLE aduio_book
ADD COLUMN is_loved BOOLEAN DEFAULT false;

ALTER TABLE audio_book
ADD COLUMN playlist_identifier UUID REFERENCES playlist (identifier);