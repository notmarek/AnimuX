CREATE TABLE public.users
(
    id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL
)

TABLESPACE pg_default;

ALTER TABLE public.users
    OWNER to postgres;