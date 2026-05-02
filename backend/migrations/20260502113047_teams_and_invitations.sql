-- Таблиця команд
CREATE TABLE teams (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
                       name TEXT NOT NULL,
                       organization TEXT,
                       contact TEXT,
                       created_by UUID NOT NULL REFERENCES users(id),
                       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Унікальність назви команди в межах ОДНОГО турніру
CREATE UNIQUE INDEX teams_tournament_lower_name_idx ON teams (tournament_id, LOWER(name));

-- Членство в командах
CREATE TABLE team_memberships (
                                  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                  team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
                                  tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
                                  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                                  role TEXT NOT NULL, -- captain, member
                                  status TEXT NOT NULL, -- invited, accepted, declined
                                  joined_at TIMESTAMPTZ,
                                  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                  UNIQUE (team_id, user_id)
);

-- Обмеження: один користувач - одна активна команда на турнір
CREATE UNIQUE INDEX one_accepted_team_per_user_per_tournament_idx
    ON team_memberships (tournament_id, user_id)
    WHERE status = 'accepted';-- Add migration script here
