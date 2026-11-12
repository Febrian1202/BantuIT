INSERT INTO
  audit_logs (
    id,
    user_id,
    action,
    module,
    module_id,
    description,
    old_data,
    new_data,
    ip_address,
    user_agent,
    created_at
  )
VALUES (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
)
RETURNING
    id,
    user_id,
    action AS "action: AuditAction",
    module AS "module: AuditModule",
    module_id,
    description,
    old_data,
    new_data,
    ip_address,
    user_agent,
    created_at

