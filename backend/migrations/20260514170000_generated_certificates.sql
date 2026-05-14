CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE generated_certificates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    snapshot JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tournament_id, team_id, user_id)
);

CREATE INDEX generated_certificates_user_id_idx ON generated_certificates(user_id);
CREATE INDEX generated_certificates_tournament_id_idx ON generated_certificates(tournament_id);
