CREATE EXTENSION IF NOT EXISTS pgcrypto;

ALTER TABLE tournaments
ADD COLUMN IF NOT EXISTS rules TEXT NOT NULL DEFAULT 'Rules will be published by organizer.';

ALTER TABLE tournaments
ADD COLUMN IF NOT EXISTS ends_at TIMESTAMPTZ NULL;

UPDATE tournaments
SET rules = 'Rules will be published by organizer.'
WHERE trim(rules) = '';

ALTER TABLE tournaments
ALTER COLUMN rules DROP DEFAULT;

ALTER TABLE tournaments
DROP CONSTRAINT IF EXISTS tournaments_title_not_empty;

ALTER TABLE tournaments
ADD CONSTRAINT tournaments_title_not_empty CHECK (length(trim(title)) > 0);

ALTER TABLE tournaments
DROP CONSTRAINT IF EXISTS tournaments_valid_registration_window;

ALTER TABLE tournaments
ADD CONSTRAINT tournaments_valid_registration_window CHECK (registration_starts_at < registration_ends_at);

ALTER TABLE tournaments
DROP CONSTRAINT IF EXISTS tournaments_registration_before_start;

ALTER TABLE tournaments
ADD CONSTRAINT tournaments_registration_before_start CHECK (registration_ends_at <= starts_at);

ALTER TABLE tournaments
DROP CONSTRAINT IF EXISTS tournaments_valid_max_teams;

ALTER TABLE tournaments
ADD CONSTRAINT tournaments_valid_max_teams CHECK (max_teams IS NULL OR max_teams > 0);

ALTER TABLE tournaments
DROP CONSTRAINT IF EXISTS tournaments_valid_status;

ALTER TABLE tournaments
ADD CONSTRAINT tournaments_valid_status CHECK (status IN ('draft', 'registration', 'running', 'finished'));

ALTER TABLE tournament_staff_roles
DROP CONSTRAINT IF EXISTS tournament_staff_roles_valid_role;

ALTER TABLE tournament_staff_roles
ADD CONSTRAINT tournament_staff_roles_valid_role CHECK (role IN ('organizer', 'jury'));

CREATE TABLE IF NOT EXISTS rounds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    task_description TEXT NOT NULL,
    technology_requirements TEXT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    starts_at TIMESTAMPTZ NOT NULL,
    deadline_at TIMESTAMPTZ NOT NULL,
    position INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT rounds_title_not_empty CHECK (length(trim(title)) > 0),
    CONSTRAINT rounds_task_description_not_empty CHECK (length(trim(task_description)) > 0),
    CONSTRAINT rounds_valid_window CHECK (starts_at < deadline_at),
    CONSTRAINT rounds_valid_position CHECK (position > 0),
    CONSTRAINT rounds_valid_status CHECK (status IN ('draft', 'active', 'submission_closed', 'evaluated')),
    CONSTRAINT rounds_unique_tournament_position UNIQUE (tournament_id, position)
);

CREATE TABLE IF NOT EXISTS round_requirements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    round_id UUID NOT NULL REFERENCES rounds(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    position INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT round_requirements_text_not_empty CHECK (length(trim(text)) > 0),
    CONSTRAINT round_requirements_valid_position CHECK (position > 0),
    CONSTRAINT round_requirements_unique_position UNIQUE (round_id, position)
);

CREATE INDEX IF NOT EXISTS idx_tournaments_status ON tournaments(status);
CREATE INDEX IF NOT EXISTS idx_tournaments_starts_at ON tournaments(starts_at);
CREATE INDEX IF NOT EXISTS idx_rounds_tournament_id ON rounds(tournament_id);
CREATE INDEX IF NOT EXISTS idx_rounds_status ON rounds(status);
