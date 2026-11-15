SELECT EXISTS(
    SELECT 1 FROM departments WHERE LOWER(name) = LOWER($1) AND deleted_at IS NULL
) AS "exists!";
