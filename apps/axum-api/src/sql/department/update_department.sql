UPDATE departments
SET name = $1, description = $2, updated_at = NOW()
WHERE id = $3
AND deleted_at IS NULL
RETURNING *;
