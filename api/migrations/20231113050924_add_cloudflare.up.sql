-- Add up migration script here
create table photo_cloudflare_resource
(
    photo_id      int not null
        constraint photo_categories_photos_id_fk
            references public.photos
            on update cascade on delete cascade,
    resource_id   uuid not null,
    constraint photo_cloudflare_resource_pk
        primary key (photo_id, resource_id)
);
