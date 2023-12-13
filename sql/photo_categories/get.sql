SELECT p.id          as "id!",
     p.title as "title!",
     p.filename as "filename!",
     p.location_taken as "location_taken!",
     p.date_taken as "date_taken!",
     p.created_at as "created_at!",
     p.updated_at as "updated_at!",
     cf.resource_id as "cloudflare_resource!"
FROM categories
         JOIN photo_categories pc on categories.id = pc.category_id
         JOIN photos p on p.id = pc.photo_id
         JOIN photo_cloudflare_resource cf ON cf.photo_id = p.id
WHERE category_id = $1

