CREATE TABLE public.anime_info
(
    id SERIAL PRIMARY KEY NOT NULL,
    real_name VARCHAR NOT NULL,
    anilist_id INTEGER,
    cover VARCHAR,
    banner VARCHAR,
    description VARCHAR,
    episodes INTEGER,
    title_preffered VARCHAR,
    title_romanji VARCHAR,
    title_original VARCHAR,
    title_english VARCHAR,
    score INTEGER,
    is_adult BOOLEAN,
    source_material VARCHAR,
    not_found BOOLEAN NOT NULL DEFAULT false
)

TABLESPACE pg_default;

ALTER TABLE public.anime_info
    OWNER to postgres;