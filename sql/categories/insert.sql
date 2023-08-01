INSERT INTO categories (name, description)
VALUES ($1, $2)
RETURNING id as "id!",
        name as "name!",
        description as "description!",
        created_at as "created_at!",
        updated_at as "updated_at!"
;