CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Таблиця команд
CREATE TABLE teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (length(trim(name)) >= 2),
    organization TEXT,
    contact TEXT,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT teams_organization_length CHECK (organization IS NULL OR length(trim(organization)) <= 120),
    CONSTRAINT teams_contact_length CHECK (contact IS NULL OR length(trim(contact)) <= 120)
);

CREATE UNIQUE INDEX teams_tournament_lower_name_idx ON teams (tournament_id, LOWER(name));
CREATE INDEX teams_tournament_id_idx ON teams (tournament_id);

-- Членство в командах
CREATE TABLE team_memberships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('captain', 'member')),
    status TEXT NOT NULL CHECK (status IN ('invited', 'accepted', 'declined', 'removed')),
    joined_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (team_id, user_id)
);

CREATE UNIQUE INDEX one_accepted_team_per_user_per_tournament_idx
    ON team_memberships (tournament_id, user_id)
    WHERE status = 'accepted';

CREATE UNIQUE INDEX one_captain_per_team_idx
    ON team_memberships (team_id)
    WHERE role = 'captain' AND status = 'accepted';

-- Інвайти в команди
CREATE TABLE team_invitations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    email TEXT NOT NULL,
    invited_user_id UUID NULL REFERENCES users(id) ON DELETE SET NULL,
    invited_by UUID NOT NULL REFERENCES users(id),
    status TEXT NOT NULL CHECK (status IN ('pending', 'accepted', 'declined', 'expired', 'cancelled')),
    token TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX team_invitations_pending_team_email_idx
    ON team_invitations (team_id, LOWER(email))
    WHERE status = 'pending';
CREATE INDEX team_invitations_invited_user_idx ON team_invitations (invited_user_id);
CREATE INDEX team_invitations_email_idx ON team_invitations (LOWER(email));
