SELECT
    (SELECT COUNT(*) FROM users WHERE department_id = $1 AND deleted_at IS NULL) AS "user_count!",
    (SELECT COUNT(*) FROM tickets WHERE department_id = $1 AND deleted_at IS NULL) AS "ticket_count!";
