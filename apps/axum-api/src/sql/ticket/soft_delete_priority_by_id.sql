UPDATE ticket_priorities
SET deleted_at = NOW()
WHERE id = $1;
