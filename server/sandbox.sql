SELECT
    JSONB_AGG (TO_JSONB (u)) AS notifications_array,
    COUNT(identifier) as total
FROM
    (
        SELECT
            *
        FROM
            notifications
        WHERE
            user_identifier = 'ebc16d3d-62a7-4dd4-bc23-11efcd2b924b'
        ORDER BY
            created_at DESC
        LIMIT
            10
        OFFSET
            0
    ) u