
CREATE TABLE categories (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE photos (
    id UUID PRIMARY KEY,
    description TEXT NOT NULL,
    date_taken TIMESTAMP NOT NULL,
    cdn_path TEXT NOT NULL
);

CREATE TABLE category_photos (
    category_id UUID NOT NULL,
    photo_id UUID NOT NULL,
    order_in_category INTEGER NOT NULL,
    FOREIGN KEY (category_id) REFERENCES categories (id),
    FOREIGN KEY (photo_id) REFERENCES photos (id),
    UNIQUE (category_id, photo_id),
    UNIQUE (category_id, order_in_category)
);
