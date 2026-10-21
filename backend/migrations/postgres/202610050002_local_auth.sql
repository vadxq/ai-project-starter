-- 保留已有 identity / Item 的 UUID 与外键；本地账号独立保存密码散列。
CREATE TABLE accounts (
    id UUID PRIMARY KEY REFERENCES identities(id),
    username TEXT NOT NULL UNIQUE CHECK (octet_length(username) BETWEEN 1 AND 64),
    password_hash TEXT NOT NULL
);

CREATE TABLE auth_sessions (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts(id),
    refresh_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ
);

CREATE INDEX auth_sessions_account ON auth_sessions(account_id);
