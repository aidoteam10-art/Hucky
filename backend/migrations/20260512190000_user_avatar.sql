ALTER TABLE users
ADD COLUMN IF NOT EXISTS avatar_url TEXT NULL;

ALTER TABLE users
DROP CONSTRAINT IF EXISTS users_avatar_url_length;

ALTER TABLE users
ADD CONSTRAINT users_avatar_url_length
CHECK (avatar_url IS NULL OR length(avatar_url) <= 1400000);
