# 更新记录

## 未发布

### 变更

- 以上游 81e089fbcc06a237353e2107bd578d20567dfba3 重建应用：Next 16.4.2、React 19.3.1、Tailwind、shadcn/ui、next-themes、类型化语言路由与原生本地检查。
- 使用上游组件和主题变量重做登录与 Item 工作区；私有请求继续在浏览器执行，并准备 standalone Node 产物。
- API 连接器适配 code/message/data 响应、上游错误与整数毫秒时间戳；客户端由新 OpenAPI 快照重新生成。

### 移除

- 当前适配排除上游演示内容与 PWA 功能。

本地格式、lint、国际化、类型、生产与独立构建、standalone HTTP Smoke 和 5 项真实浏览器测试通过。生产依赖 audit 无漏洞；上游开发工具告警已记录。
