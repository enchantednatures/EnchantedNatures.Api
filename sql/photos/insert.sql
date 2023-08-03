INSERT INTO photos (name, description, url)
VALUES ($1, $2, $3) RETURNING id as "id!",
               name as "name!",
               description as "description!",
               url as "url!",
               created_at as "created_at!",
               updated_at as "updated_at!"