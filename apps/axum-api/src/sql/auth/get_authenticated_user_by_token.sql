SELECT
    u.id,
    u.role_id,
    u.department_id,
    u.email,
    u.password,
    u.full_name,
    u.status AS "status: UserStatus",
    u.must_change_password,
    u.last_login_at,
    u.created_at,
    u.updated_at,
    u.deleted_at,
    r.name AS "role_name: RoleName"
FROM personal_access_tokens pat
JOIN users u ON u.id = pat.tokenable_id
JOIN roles r on r.id = u.role_id
WHERE pat.token = $1
AND pat.created_at >= $2
AND u.deleted_at IS NULL
AND u.status = 'active'
LIMIT 1;
