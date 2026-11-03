SELECT
    id,
    ticket_number,
    title,
    description,
    category_id,
    priority_id,
    status_id,
    reporter_id,
    technician_id,
    department_id,
    asset_id,
    sla_duration_minutes,
    sla_deadline,
    sla_breached,
    sla_breached_at,
    resolved_at,
    closed_at,
    created_at,
    updated_at,
    deleted_at
FROM tickets
WHERE id = $1 AND deleted_at IS NULL;
