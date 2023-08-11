SELECT id as "id!",
       title as "title!",
       filename as "filename!",
       location_taken as "location_taken!",
       date_taken as "date_taken!",
       created_at as "created_at!",
       updated_at as "updated_at!"
FROM photos
WHERE id = $1;
