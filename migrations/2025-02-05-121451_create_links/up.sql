-- Your SQL goes here
CREATE TABLE links (
    link_id SERIAL PRIMARY KEY,
    created_at DATE NOT NULL DEFAULT CURRENT_DATE,
    source VARCHAR(32) NOT NULL,
    alias VARCHAR(32) NOT NULL
)