-- Add up migration script here
ALTER TABLE users
    ADD COLUMN username varchar NOT NULL DEFAULT '',
    ADD COLUMN bio      varchar,
    ADD COLUMN avatar   varchar,
    ADD COLUMN work     varchar;