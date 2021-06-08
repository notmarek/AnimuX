CREATE TABLE public.invites
(
    id SERIAL PRIMARY KEY NOT NULL,
    invite VARCHAR NOT NULL,
    used BOOLEAN NOT NULL DEFAULT false
)

TABLESPACE pg_default;

ALTER TABLE public.invites
    OWNER to postgres;