-- 保持与 PostgreSQL 相同迁移版本；字段说明已随建表定义。
ALTER TABLE items COMMENT='按 owner 隔离、使用 version 并发控制的个人任务';
