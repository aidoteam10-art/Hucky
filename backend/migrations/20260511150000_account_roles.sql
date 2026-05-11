ALTER TABLE users
ADD COLUMN IF NOT EXISTS account_role TEXT NOT NULL DEFAULT 'participant';

UPDATE users u
SET account_role = 'organiser'
WHERE account_role = 'participant'
    AND (
        EXISTS (
            SELECT 1
            FROM tournaments t
            WHERE t.organizer_id = u.id
        )
        OR EXISTS (
            SELECT 1
            FROM tournament_staff_roles tsr
            WHERE tsr.user_id = u.id
                AND tsr.role = 'organizer'
        )
    );

UPDATE users u
SET account_role = 'jury'
WHERE account_role = 'participant'
    AND EXISTS (
        SELECT 1
        FROM tournament_staff_roles tsr
        WHERE tsr.user_id = u.id
            AND tsr.role = 'jury'
    );

ALTER TABLE users
DROP CONSTRAINT IF EXISTS users_account_role_valid;

ALTER TABLE users
ADD CONSTRAINT users_account_role_valid
CHECK (account_role IN ('participant', 'organiser', 'jury', 'admin'));

CREATE INDEX IF NOT EXISTS idx_users_account_role ON users(account_role);
