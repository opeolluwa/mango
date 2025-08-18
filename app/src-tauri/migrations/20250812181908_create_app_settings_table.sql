-- Add migration script here
-- Create a new table that stores app settings --
CREATE TABLE
    IF NOT EXISTS app_settings (app_initialized BOOLEAN DEFAULT 0);