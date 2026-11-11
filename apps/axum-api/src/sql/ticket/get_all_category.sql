SELECT
  c.id,
  c.name,
  c.description,
  c.parent_id,
  c.created_at,
  c.updated_at,
  c.deleted_at,
  CASE
    WHEN p.id IS NOT NULL THEN json_build_object (
      'id',
      p.id,
      'name',
      p.name,
      'description',
      p.description,
      'parent_id',
      p.parent_id,
      'created_at',
      p.created_at,
      'updated_at',
      p.updated_at
    )
    ELSE NULL
  END AS "parent: Json<ParentCategory>"
FROM
  ticket_categories c
  LEFT JOIN ticket_categories p ON p.id = c.parent_id
  AND p.deleted_at IS NULL
WHERE
  c.deleted_at IS NULL
ORDER BY
  c.name ASC;
