SELECT (SELECT COUNT(*) FROM tickets WHERE priority_id = $1 AND deleted_at IS NULL) AS "ticket_count!";
