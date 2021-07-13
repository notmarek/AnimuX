CREATE TABLE public.naughty
(
    id SERIAL PRIMARY KEY NOT NULL,
    ip VARCHAR NOT NULL,
    times INTEGER NOT NULL
)

TABLESPACE pg_default;

ALTER TABLE public.naughty
    OWNER to postgres;