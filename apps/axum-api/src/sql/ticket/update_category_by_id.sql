
WITH updated AS(
    UPDATE ticket_categories
    SET name = $1, description = $2, parent_id = $3, updated_at = NOW()
    WHERE id = $4 AND deleted_at IS NULL
    RETURNING *
)
SELECT
    c.id AS "id!",
    c.name AS "name!",
    c.description,
    c.parent_id,
    c.created_at AS "created_at!",
    c.updated_at AS "updated_at!",
    c.deleted_at,
    CASE
        WHEN p.id IS NOT NULL THEN json_build_object(
            'id', p.id,
            'name', p.name,
            'description', p.description,
            'parent_id', p.parent_id,
            'created_at', p.created_at,
            'updated_at', p.updated_at
        )
        ELSE NULL
    END AS "parent: Json<ParentCategory>"
FROM updated c
LEFT JOIN ticket_categories p 
    ON p.id = c.parent_id 
    AND p.deleted_at IS NULL;
