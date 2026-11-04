WITH updated_user AS (
    UPDATE users
    SET
        last_login_at = CURRENT_TIMESTAMP,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = $1
)
INSERT INTO personal_access_tokens (
    tokenable_id,
    token,
    name,
    abilities,
    created_at,
    updated_at
)
VALUES (
    $1,
    $2,
    'auth_token',
    '["*"]',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);
