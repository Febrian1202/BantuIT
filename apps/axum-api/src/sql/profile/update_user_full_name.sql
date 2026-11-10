UPDATE users u
SET full_name = $1, updated_at = NOW()
FROM roles r
WHERE u.role_id = r.id
AND u.id = $2
AND u.deleted_at IS NULL
AND u.status = 'active'
RETURNING 
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
    r.name AS "role_name: RoleName";

