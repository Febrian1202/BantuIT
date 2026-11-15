SELECT 1 FROM ticket_priorities WHERE LOWER(name) = LOWER($1) AND deleted_at IS NULL LIMIT 1
