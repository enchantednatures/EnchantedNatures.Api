-- Add up migration script here
create table photos
(
    id          serial                                 not null
        constraint photos_pk
            primary key,
    title        varchar(255)                           not null,
    filename    varchar(255)                           not null,
    location_taken        varchar(255)                           not null,
    date_taken        date not null,   
    created_at  timestamp with time zone default now() not null,
    updated_at  timestamp with time zone default now() not null
);

create table categories
(
    id          serial                                 not null
        constraint categories_pk
            primary key,
    name        varchar(255)                           not null,
    created_at  timestamp with time zone default now() not null,
    updated_at  timestamp with time zone default now() not null
);


create table photo_categories
(
    photo_id      int not null
        constraint photo_categories_photos_id_fk
            references public.photos
            on update cascade on delete cascade,
    category_id   int not null
        constraint photo_categories_categories_id_fk
            references public.categories
            on update cascade on delete cascade,
    display_order int not null,
    constraint photo_categories_pk
        primary key (photo_id, category_id),
    constraint photo_categories_photo_id_category_id_display_order_key unique (category_id, display_order)

);
