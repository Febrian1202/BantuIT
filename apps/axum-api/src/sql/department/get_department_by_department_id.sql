SELECT *
FROM departments
WHERE id = $1
AND deleted_at IS NULL;
