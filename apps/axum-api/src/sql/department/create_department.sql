INSERT INTO departments (name, description, created_at, updated_at)
VALUES ($1, $2, NOW(), NOW())
RETURNING *;
