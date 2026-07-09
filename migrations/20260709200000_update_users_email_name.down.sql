ALTER TABLE users
    DROP INDEX IF EXISTS users_email_unique_idx;

ALTER TABLE users
    DROP COLUMN email;
