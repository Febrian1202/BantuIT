SELECT
    (SELECT COUNT(*) FROM tickets WHERE category_id = $1 AND deleted_at IS NULL) AS "ticket_count!",
    (SELECT COUNT(*) FROM ticket_categories WHERE parent_id = $1 AND deleted_at IS NULL) AS "child_count!";
