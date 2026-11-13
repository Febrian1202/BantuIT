SELECT 1 FROM ticket_categories WHERE name = $1 AND deleted_at IS NULL LIMIT 1
