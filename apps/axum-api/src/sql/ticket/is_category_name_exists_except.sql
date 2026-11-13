SELECT EXISTS( 
    SELECT 1 FROM ticket_categories
    WHERE LOWER(name) = LOWER($1)
    AND id != $2
    AND deleted_at IS NULL
) AS "exists!";
