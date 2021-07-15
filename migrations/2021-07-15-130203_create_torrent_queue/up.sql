CREATE TABLE public.torrent_queue
(
    id SERIAL PRIMARY KEY NOT NULL,
    link VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL
)

TABLESPACE pg_default;

ALTER TABLE public.torrent_queue
    OWNER to postgres;