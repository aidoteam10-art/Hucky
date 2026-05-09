-- 1. Таблиця користувачів (завдання людини 1)
CREATE TABLE users (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       email TEXT NOT NULL UNIQUE,
                       full_name TEXT NOT NULL,
                       password_hash TEXT NOT NULL,
                       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Таблиця турнірів (завдання людини 2)
CREATE TABLE tournaments (
                             id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                             organizer_id UUID NOT NULL REFERENCES users(id),
                             title TEXT NOT NULL,
                             description TEXT NOT NULL,
                             status TEXT NOT NULL DEFAULT 'draft', -- draft, registration, running, finished
                             registration_starts_at TIMESTAMPTZ NOT NULL,
                             registration_ends_at TIMESTAMPTZ NOT NULL,
                             starts_at TIMESTAMPTZ NOT NULL,
                             max_teams INTEGER,
                             created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                             updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 3. Таблиця ролей стафу (організатори/журі)
CREATE TABLE tournament_staff_roles (
                                        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                        tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
                                        user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                                        role TEXT NOT NULL, -- organizer, jury
                                        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                        UNIQUE (tournament_id, user_id, role)
);-- Add migration script here



-- Таблиця раундів (імітація роботи Людини 2)
CREATE TABLE rounds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    task_description TEXT NOT NULL,
    technology_requirements TEXT,
    status TEXT NOT NULL DEFAULT 'draft',
    starts_at TIMESTAMPTZ NOT NULL,
    deadline_at TIMESTAMPTZ NOT NULL,
    position INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Таблиця команд (імітація роботи Людини 3)
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

-- Таблиця учасників команди (імітація роботи Людини 3)
CREATE TABLE team_memberships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    status TEXT NOT NULL,
    joined_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (team_id, user_id)
);