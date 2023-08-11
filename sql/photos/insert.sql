INSERT INTO photos (title, filename, description, location_taken, date_taken)
VALUES ($1, $2, $3, $4, $5) RETURNING id as "id!",
       title as "title!",
       filename as "filename!",
       description as "description!",
       location_taken as "location_taken!",
       date_taken as "date_taken!",
       created_at as "created_at!",
       updated_at as "updated_at!"
               
