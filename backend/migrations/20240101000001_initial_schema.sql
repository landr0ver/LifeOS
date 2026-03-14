-- LifeOS Initial Schema

-- Sessions (auth)
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL
);

-- Kanban
CREATE TABLE boards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE columns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    board_id UUID REFERENCES boards(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    position INT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    column_id UUID REFERENCES columns(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    position INT NOT NULL,
    due_date TIMESTAMPTZ,
    labels TEXT[],
    github_issue_id INT,
    github_pr_id INT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

-- Habits
CREATE TABLE habits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    frequency TEXT NOT NULL CHECK (frequency IN ('daily', 'weekly', 'custom')),
    target_days INT[] DEFAULT '{1,2,3,4,5,6,7}',
    color TEXT,
    icon TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    archived_at TIMESTAMPTZ
);

CREATE TABLE habit_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    habit_id UUID REFERENCES habits(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    completed BOOLEAN DEFAULT false,
    note TEXT,
    UNIQUE(habit_id, date)
);

-- Notes (Knowledge Graph)
CREATE TABLE notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    tags TEXT[],
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE note_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_id UUID REFERENCES notes(id) ON DELETE CASCADE,
    target_id UUID REFERENCES notes(id) ON DELETE CASCADE,
    label TEXT,
    UNIQUE(source_id, target_id)
);

CREATE INDEX notes_search_idx ON notes USING GIN (
    to_tsvector('english', title || ' ' || content)
);

-- Calendar
CREATE TABLE events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    all_day BOOLEAN DEFAULT false,
    recurrence TEXT,
    color TEXT,
    card_id UUID REFERENCES cards(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

-- GitHub Integration
CREATE TABLE github_repos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner TEXT NOT NULL,
    name TEXT NOT NULL,
    github_token_encrypted TEXT NOT NULL,
    last_synced_at TIMESTAMPTZ,
    UNIQUE(owner, name)
);

CREATE TABLE github_issues (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repo_id UUID REFERENCES github_repos(id) ON DELETE CASCADE,
    issue_number INT NOT NULL,
    title TEXT NOT NULL,
    state TEXT NOT NULL,
    labels TEXT[],
    body TEXT,
    card_id UUID REFERENCES cards(id) ON DELETE SET NULL,
    synced_at TIMESTAMPTZ DEFAULT now(),
    UNIQUE(repo_id, issue_number)
);

CREATE TABLE github_pull_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repo_id UUID REFERENCES github_repos(id) ON DELETE CASCADE,
    pr_number INT NOT NULL,
    title TEXT NOT NULL,
    state TEXT NOT NULL,
    linked_issue_number INT,
    card_id UUID REFERENCES cards(id) ON DELETE SET NULL,
    synced_at TIMESTAMPTZ DEFAULT now(),
    UNIQUE(repo_id, pr_number)
);

CREATE TABLE feature_sync_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feature_title TEXT NOT NULL,
    issue_number INT,
    repo_id UUID REFERENCES github_repos(id),
    synced_at TIMESTAMPTZ DEFAULT now()
);
