UPDATE users
SET password = $1, must_change_password = false, updated_at = CURRENT_TIMESTAMP
WHERE id = $2;
