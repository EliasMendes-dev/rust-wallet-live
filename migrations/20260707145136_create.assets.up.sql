-- Add up migration script here
CREATE TABLE IF NOT EXISTS assets (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    unit_value DOUBLE PRECISION NOT NULL
);