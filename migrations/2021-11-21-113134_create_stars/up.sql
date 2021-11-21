CREATE TABLE public.stars
(
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    path TEXT NOT NULL
)

TABLESPACE pg_default;

ALTER TABLE public.stars
    OWNER to postgres;