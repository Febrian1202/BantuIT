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
FROM
  users u
  JOIN roles r ON r.id = u.role_id
WHERE u.email = $1 AND u.deleted_at IS NULL
