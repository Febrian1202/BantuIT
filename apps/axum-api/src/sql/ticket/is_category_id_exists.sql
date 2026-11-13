SELECT 1 FROM ticket_categories WHERE id = $1 AND deleted_at IS NULL LIMIT 1
