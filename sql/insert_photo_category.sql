INSERT INTO public.photo_categories (photo_id, category_id, display_order)
VALUES (1, 1,
        ((SELECT coalesce(max(display_order), 0) as max_display_order
          FROM photo_categories
          WHERE category_id = 1
          GROUP BY category_id) + 1
            ))