CREATE TABLE identities (
    id BINARY(16) PRIMARY KEY COMMENT '用户 UUID',
    issuer VARCHAR(255) NOT NULL COMMENT '身份来源',
    subject VARCHAR(255) NOT NULL COMMENT '来源中的身份标识',
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间（UTC）',
    UNIQUE (issuer, subject)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='身份';

CREATE TABLE items (
    id BINARY(16) PRIMARY KEY COMMENT '任务 UUID',
    owner_id BINARY(16) NOT NULL COMMENT '所属用户 UUID',
    title TEXT NOT NULL COMMENT '裁剪后 1–240 UTF-8 字节',
    completed BOOLEAN NOT NULL COMMENT '是否完成',
    version INTEGER NOT NULL COMMENT '乐观版本',
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '创建时间（UTC）',
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT '修改时间（UTC）',
    CONSTRAINT items_owner_fk FOREIGN KEY (owner_id) REFERENCES identities(id),
    CHECK (OCTET_LENGTH(title) BETWEEN 1 AND 240),
    CHECK (version > 0),
    INDEX items_owner_created (owner_id, created_at DESC, id DESC)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='个人任务';
