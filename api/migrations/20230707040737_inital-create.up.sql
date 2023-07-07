-- Add up migration script here
create table public.photos
(
    id serial not null
        constraint photos_pk
            primary key,
    name varchar(255) not null,
    description text,
    url varchar(255) not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

create table public.categories (
    id serial not null
        constraint categories_pk
            primary key,
    name varchar(255) not null,
    description text,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);


create table photo_categories (
    photo_id int not null
        constraint photo_categories_photos_id_fk
            references public.photos
                on update cascade on delete cascade,
    category_id int not null
        constraint photo_categories_categories_id_fk
            references public.categories
                on update cascade on delete cascade,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null,
    constraint photo_categories_pk
        primary key (photo_id, category_id)
);
