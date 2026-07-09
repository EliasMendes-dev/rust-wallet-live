-- Add up migration script here
CREATE TABLE IF NOT EXISTS assets (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    unit_value DOUBLE PRECISION NOT NULL
);

INSERT INTO assets (name, unit_value)
VALUES
    ('Bitcoin', 95.00),
    ('Ethereum', 82.50),
    ('Solana', 68.90),
    ('Cardano', 24.75),
    ('XRP', 12.90),
    ('Petrobras', 36.40),
    ('Vale', 58.90),
    ('Banco do Brasil', 29.80),
    ('Nubank', 27.50),
    ('Tesla', 99.90);