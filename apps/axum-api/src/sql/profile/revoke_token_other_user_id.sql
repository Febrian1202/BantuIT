DELETE FROM personal_access_tokens
WHERE tokenable_id = $1 AND token != $2;
