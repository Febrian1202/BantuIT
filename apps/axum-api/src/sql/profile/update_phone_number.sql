UPDATE employee_profiles
SET phone = $1, updated_at = NOW()
WHERE user_id = $2
RETURNING *;
