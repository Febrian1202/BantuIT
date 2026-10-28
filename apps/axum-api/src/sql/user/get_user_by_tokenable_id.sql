SELECT
    u.id, u.role_id, u.department_id, u.email, u.password, u.full_name,
    u.status, u.must_change_password, u.last_login_at, u.created_at, u.updated_at, u.deleted_at, r.name AS role_name
FROM users u
JOIN roles r ON r.id = u.role_id
WHERE u.id = $1 AND u.deleted_at IS NULL AND u.status = 'active'
