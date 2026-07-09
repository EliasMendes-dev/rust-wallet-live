ALTER TABLE users
    ADD COLUMN email TEXT;

UPDATE users
    SET email = username || '@example.com'
    WHERE email IS NULL;

ALTER TABLE users
    ALTER COLUMN email SET NOT NULL;

ALTER TABLE users
    DROP CONSTRAINT IF EXISTS users_username_key;

CREATE UNIQUE INDEX IF NOT EXISTS users_email_unique_idx ON users(email);