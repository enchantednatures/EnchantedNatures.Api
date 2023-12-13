SELECT id as "id!",
       title as "title!",
       filename as "filename!",
       location_taken as "location_taken!",
       date_taken as "date_taken!",
       created_at as "created_at!",
       updated_at as "updated_at!",
       resource_id as "cloudflare_resource!"
FROM photos
JOIN photo_cloudflare_resource cf ON cf.photo_id = photos.id
WHERE id = $1;
