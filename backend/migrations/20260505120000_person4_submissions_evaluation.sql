CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE submissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    round_id UUID NOT NULL REFERENCES rounds(id) ON DELETE CASCADE,
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    github_url TEXT NOT NULL,
    video_demo_url TEXT NOT NULL,
    live_demo_url TEXT,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'submitted' CHECK (status IN ('submitted', 'locked')),
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    locked_at TIMESTAMPTZ,
    CONSTRAINT submissions_description_length CHECK (description IS NULL OR length(description) <= 2000),
    UNIQUE (round_id, team_id)
);

CREATE TABLE evaluation_criteria (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    max_score INTEGER NOT NULL DEFAULT 100,
    weight NUMERIC NOT NULL DEFAULT 1.0,
    position INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT evaluation_criteria_code_not_empty CHECK (length(trim(code)) > 0),
    CONSTRAINT evaluation_criteria_name_not_empty CHECK (length(trim(name)) > 0),
    CONSTRAINT evaluation_criteria_max_score_positive CHECK (max_score > 0),
    CONSTRAINT evaluation_criteria_weight_positive CHECK (weight > 0),
    CONSTRAINT evaluation_criteria_position_positive CHECK (position > 0),
    UNIQUE (tournament_id, code),
    UNIQUE (tournament_id, position)
);

CREATE TABLE jury_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    round_id UUID NOT NULL REFERENCES rounds(id) ON DELETE CASCADE,
    submission_id UUID NOT NULL REFERENCES submissions(id) ON DELETE CASCADE,
    jury_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'completed')),
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    UNIQUE (submission_id, jury_user_id)
);

CREATE TABLE evaluation_scores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assignment_id UUID NOT NULL REFERENCES jury_assignments(id) ON DELETE CASCADE,
    criterion_id UUID NOT NULL REFERENCES evaluation_criteria(id) ON DELETE CASCADE,
    score INTEGER NOT NULL CHECK (score >= 0 AND score <= 100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (assignment_id, criterion_id)
);

CREATE TABLE evaluation_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assignment_id UUID NOT NULL UNIQUE REFERENCES jury_assignments(id) ON DELETE CASCADE,
    comment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT evaluation_comments_length CHECK (comment IS NULL OR length(comment) <= 2000)
);

CREATE INDEX idx_submissions_round_id ON submissions(round_id);
CREATE INDEX idx_submissions_team_id ON submissions(team_id);
CREATE INDEX idx_jury_assignments_round_id ON jury_assignments(round_id);
CREATE INDEX idx_jury_assignments_jury_user_id ON jury_assignments(jury_user_id);
CREATE INDEX idx_jury_assignments_status ON jury_assignments(status);
CREATE INDEX idx_evaluation_scores_criterion_id ON evaluation_scores(criterion_id);

CREATE OR REPLACE FUNCTION seed_default_evaluation_criteria_for_tournament()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO evaluation_criteria
        (tournament_id, code, name, description, max_score, weight, position)
    VALUES
        (NEW.id, 'backend', 'Backend', 'Clean backend code, architecture, and tests', 100, 1.0, 1),
        (NEW.id, 'database', 'Database', 'Database structure, constraints, and relationships', 100, 1.0, 2),
        (NEW.id, 'frontend', 'Frontend', 'Frontend code quality, UX, UI, and tests', 100, 1.0, 3),
        (NEW.id, 'requirements', 'Requirements', 'Completion of required tournament features', 100, 1.0, 4),
        (NEW.id, 'functionality', 'Functionality', 'Working behavior, reliability, and absence of critical bugs', 100, 1.0, 5),
        (NEW.id, 'usability', 'Usability', 'Ease of use and clarity for participants, organizers, and jury', 100, 1.0, 6)
    ON CONFLICT (tournament_id, code) DO NOTHING;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS seed_default_evaluation_criteria_after_tournament_insert ON tournaments;

CREATE TRIGGER seed_default_evaluation_criteria_after_tournament_insert
AFTER INSERT ON tournaments
FOR EACH ROW
EXECUTE FUNCTION seed_default_evaluation_criteria_for_tournament();

INSERT INTO evaluation_criteria
    (tournament_id, code, name, description, max_score, weight, position)
SELECT
    t.id,
    criteria.code,
    criteria.name,
    criteria.description,
    100,
    1.0,
    criteria.position
FROM tournaments t
CROSS JOIN (
    VALUES
        ('backend', 'Backend', 'Clean backend code, architecture, and tests', 1),
        ('database', 'Database', 'Database structure, constraints, and relationships', 2),
        ('frontend', 'Frontend', 'Frontend code quality, UX, UI, and tests', 3),
        ('requirements', 'Requirements', 'Completion of required tournament features', 4),
        ('functionality', 'Functionality', 'Working behavior, reliability, and absence of critical bugs', 5),
        ('usability', 'Usability', 'Ease of use and clarity for participants, organizers, and jury', 6)
) AS criteria(code, name, description, position)
ON CONFLICT (tournament_id, code) DO NOTHING;
