-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    username text NOT NULL,
    address text NOT NULL,
    joined_at timestamp NOT NULL
);
