CREATE TABLE accounts (
    id BINARY(16) PRIMARY KEY COMMENT '身份 UUID',
    username VARCHAR(64) NOT NULL UNIQUE COMMENT '区分大小写的用户名',
    password_hash TEXT NOT NULL COMMENT 'Argon2id 密码散列',
    CONSTRAINT accounts_identity_fk FOREIGN KEY (id) REFERENCES identities(id),
    CHECK (OCTET_LENGTH(username) BETWEEN 1 AND 64)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='本地账号';

CREATE TABLE auth_sessions (
    id BINARY(16) PRIMARY KEY COMMENT '会话 UUID',
    account_id BINARY(16) NOT NULL COMMENT '账号 UUID',
    refresh_hash VARCHAR(64) NOT NULL UNIQUE COMMENT '刷新凭证 SHA-256 散列',
    expires_at TIMESTAMP(3) NOT NULL COMMENT '绝对过期时间（UTC）',
    revoked_at TIMESTAMP(3) NULL COMMENT '撤销时间（UTC）',
    CONSTRAINT sessions_account_fk FOREIGN KEY (account_id) REFERENCES accounts(id),
    INDEX auth_sessions_account (account_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='可撤销的认证会话';
