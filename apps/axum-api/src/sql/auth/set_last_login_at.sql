UPDATE users
SET last_login_at = $1
WHERE id = $2;

