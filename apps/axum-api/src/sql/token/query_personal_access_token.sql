SELECT tokenable_id, created_at
FROM personal_access_tokens
WHERE token = $1
AND created_at >= $2
LIMIT 1;
