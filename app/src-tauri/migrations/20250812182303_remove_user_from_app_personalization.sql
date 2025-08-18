-- Add migration script here
-- Remove user-related fields from app_personalization table --
CREATE TABLE
    IF NOT EXISTS app_personalization_new (
        theme TEXT DEFAULT 'light',
        language TEXT,
        preferred_voice TEXT
    );

INSERT INTO
    app_personalization_new (theme, language, preferred_voice)
SELECT
    theme,
    language,
    preferred_voice
FROM
    app_personalization;

DROP TABLE app_personalization;

ALTER TABLE app_personalization_new
RENAME TO app_personalization;