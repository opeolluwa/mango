INSERT INTO
    audio_books (
        identifier,
        name,
        src,
        user_identifier,
        playlist_identifier
    )
VALUES
    ($1, $2, $3, $4);