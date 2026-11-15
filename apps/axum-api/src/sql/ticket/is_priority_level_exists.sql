SELECT 1 FROM ticket_priorities WHERE level = $1 AND deleted_at IS NULL LIMIT 1
