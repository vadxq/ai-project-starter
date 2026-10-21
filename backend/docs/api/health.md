# health 接口文档

## 说明

health 只负责探针类接口：

- `/health`：检查进程是否存活
- `/ready`：实际查询当前选择的 PostgreSQL / MySQL；超时或不可用时返回 503，数据库恢复后无需重启

默认本地地址使用 `http://localhost:8080`。

默认不配置数据库。通过 `APP_DATABASE__URL` 选择数据库；`APP_DATABASE__TIMEOUT_SECS` 默认 3 秒，覆盖连接获取与查询。响应均包含 `x-request-id`。

## 路由清单

| 方法  | 路径      | 鉴权 | 说明     |
| ----- | --------- | ---- | -------- |
| `GET` | `/health` | 无   | 存活探针 |
| `GET` | `/ready`  | 无   | 就绪探针 |

## 存活检查

```bash
curl http://localhost:8080/health
```

预期返回：

```json
{
  "code": "200",
  "message": "success",
  "data": {
    "status": "ok"
  }
}
```

## 就绪检查

```bash
curl http://localhost:8080/ready
```

数据库可用时预期返回：

```json
{
  "code": "200",
  "message": "success",
  "data": {
    "ready": true,
    "database": "ready"
  }
}
```

数据库未配置时返回：

```json
{
  "code": "503",
  "message": "database not configured"
}
```

数据库连接不可用时返回：

```json
{
  "code": "503",
  "message": "database not ready"
}
```
