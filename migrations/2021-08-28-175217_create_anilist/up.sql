CREATE TABLE public.anilist
(
    id SERIAL PRIMARY KEY NOT NULL,
    anime_name VARCHAR NOT NULL,
    anilist_id INTEGER,
    preview_image VARCHAR,
    not_found BOOLEAN NOT NULL DEFAULT false
)

TABLESPACE pg_default;

ALTER TABLE public.anilist
    OWNER to postgres;