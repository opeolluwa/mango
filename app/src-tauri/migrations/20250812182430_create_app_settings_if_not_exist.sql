-- Add migration script here
CREATE TABLE
    IF NOT EXISTS app_settings (app_initialized BOOLEAN DEFAULT FALSE);