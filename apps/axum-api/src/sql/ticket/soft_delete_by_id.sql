UPDATE ticket_categories
SET deleted_at = NOW()
WHERE id = $1;
