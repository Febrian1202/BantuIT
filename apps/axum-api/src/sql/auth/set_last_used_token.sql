UPDATE personal_access_tokens
SET last_used_at = NOW()
WHERE token = $1;
