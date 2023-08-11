INSERT INTO categories (name)
VALUES ($1)
RETURNING id as "id!",
        name as "name!",
        created_at as "created_at!",
        updated_at as "updated_at!";
