SELECT photos.id as "id!",
       photos.title as "title!",
       photos.filename as "filename!",
       photos.location_taken as "location_taken!",
       photos.date_taken as "date_taken!",
       photos.created_at as "created_at!",
       photos.updated_at as "updated_at!",
       cf.resource_id as "cloudflare_resource!"
FROM photos
JOIN photo_cloudflare_resource cf ON cf.photo_id = photos.id
