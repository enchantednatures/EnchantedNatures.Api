-- Add down migration script here

DELETE FROM categories
WHERE id < 1000;

DELETE FROM photos
WHERE id < 1000;
