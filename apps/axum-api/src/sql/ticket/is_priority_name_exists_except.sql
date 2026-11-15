SELECT EXISTS( 
    SELECT 1 FROM ticket_priorities
    WHERE LOWER(name) = LOWER($1)
    AND id != $2
    AND deleted_at IS NULL
) as "exists!";
