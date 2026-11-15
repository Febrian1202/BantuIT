SELECT
  *
FROM
  departments
WHERE
  deleted_at IS NULL
ORDER BY
  name;
