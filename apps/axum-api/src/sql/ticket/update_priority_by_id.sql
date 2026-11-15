UPDATE ticket_priorities
SET name = $1, level = $2, sla_minutes = $3, description = $4, updated_at = NOW()
WHERE id = $5 AND deleted_at IS NULL
RETURNING *;
