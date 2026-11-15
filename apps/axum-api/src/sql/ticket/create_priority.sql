INSERT INTO ticket_priorities (name, level, sla_minutes, description, created_at, updated_at)
VALUES ($1, $2, $3, $4, NOW(), NOW())
RETURNING *;
