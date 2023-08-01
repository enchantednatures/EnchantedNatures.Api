SELECT id as "id!",
       name as "name!",
       url as "url!",
       description as "description!",
       created_at as "created_at!",
       updated_at as "updated_at!"
FROM photos
WHERE id = $1;