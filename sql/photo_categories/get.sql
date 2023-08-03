SELECT p.id          as "id!",
       p.name        as "name!",
       p.description as "description!",
       p.url         as "url!",
       p.created_at  as "created_at!",
       p.updated_at  as "updated_at!"
FROM categories
         JOIN photo_categories pc on categories.id = pc.category_id
         JOIN photos p on p.id = pc.photo_id
WHERE category_id = $1