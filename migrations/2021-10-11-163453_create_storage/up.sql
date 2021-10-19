CREATE TABLE public.storage
(
    id SERIAL PRIMARY KEY NOT NULL,
    paths TEXT[] NOT NULL,
    name VARCHAR NOT NULL,
    exceptions TEXT[] NOT NULL
)

TABLESPACE pg_default;

ALTER TABLE public.storage
    OWNER to postgres;