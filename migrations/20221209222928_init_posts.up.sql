CREATE TABLE posts(
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    author_id uuid NOT NULL,
    title text NOT NULL,
    content text NOT NULL,
    created_at timestamp NOT NULL,
    edited_at timestamp NOT NULL,
    CONSTRAINT fk_user_id
    FOREIGN KEY(author_id)
    REFERENCES users(id)
);

