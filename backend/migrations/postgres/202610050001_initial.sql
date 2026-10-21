CREATE TABLE identities (
    id UUID PRIMARY KEY,
    issuer TEXT NOT NULL,
    subject TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (issuer, subject)
);

CREATE TABLE items (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES identities(id),
    title TEXT NOT NULL CHECK (octet_length(title) BETWEEN 1 AND 240),
    completed BOOLEAN NOT NULL,
    version INTEGER NOT NULL CHECK (version > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX items_owner_created ON items (owner_id, created_at DESC, id DESC);
