SELECT
  id,
  name,
  level,
  sla_minutes,
  description,
  created_at,
  updated_at,
  deleted_at
FROM
  ticket_priorities
WHERE
  deleted_at IS NULL
ORDER BY
  sla_minutes
