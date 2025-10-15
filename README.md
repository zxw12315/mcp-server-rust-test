# MCP Server

这是一个用 Rust 编写的简单 MCP (Multi-Client Protocol) 服务器，适用于本地局域网使用。

## 功能特点

- 基于 Warp 框架构建的异步 HTTP 服务器
- 支持 MCP 协议消息处理
- 提供健康检查端点
- 内置Web管理界面
- 静态文件服务
- 易于扩展和定制

## 技术栈

- Rust
- Tokio 异步运行时
- Warp HTTP 框架
- Serde 序列化/反序列化

## 文档目录

- [用户手册](docs/USER_MANUAL.md) - 详细的使用指南
- [技术文档](docs/MCP_SERVER_TECHNICAL_DOCUMENTATION.md) - 开发框架和实现细节
- [架构说明](docs/ARCHITECTURE.md) - 系统架构和设计说明
- [部署指南](docs/DEPLOYMENT_GUIDE.md) - 部署和运维指南
- [API文档](API_DOCUMENTATION.md) - API端点详细说明

## 快速开始

1. 确保已安装 Rust 和 Cargo
2. 克隆项目
3. 运行服务器：

```bash
cargo run
```

服务器将在以下端口启动：
- Web界面: `http://localhost:3030`
- API服务: `http://localhost:3032`

## API 端点

详细API文档请参阅 [API_DOCUMENTATION.md](API_DOCUMENTATION.md)

- `POST /mcp` - 处理 MCP 消息 (端口 3032)
- `GET /health` - 健康检查端点 (端口 3032)
- `GET /` - Web管理界面 (端口 3030)
- `GET /static/` - 静态文件服务 (端口 3030)
- `GET /api/messages` - 获取消息历史 (端口 3030)
- `POST /api/clear` - 清除消息历史 (端口 3030)

## Web 界面

MCP服务器包含一个内置的Web管理界面，可通过 `http://localhost:3030` 访问。该界面提供以下功能：

- 实时显示服务器状态
- 查看MCP消息历史
- 清除消息历史
- API端点信息展示
- 在线测试MCP消息发送

## 示例请求

### 健康检查

```bash
curl http://localhost:3032/health
```

### 发送 MCP 消息

```bash
curl -X POST http://localhost:3032/mcp \
  -H "Content-Type: application/json" \
  -d '{"id": "1", "method": "echo", "params": {"message": "Hello MCP"}}'
```

### PowerShell 示例

在 Windows PowerShell 中使用 Invoke-WebRequest:

```powershell
# 健康检查
Invoke-WebRequest -Uri http://localhost:3032/health -Method GET

# 发送 MCP 消息
$body = '{"id": "1", "method": "echo", "params": {"message": "Hello MCP"}}'
Invoke-WebRequest -Uri http://localhost:3032/mcp -Method POST -Body $body -ContentType "application/json"
```

## 扩展建议

1. 添加身份验证和授权机制
2. 实现更复杂的 MCP 消息处理逻辑
3. 添加数据库支持
4. 实现 WebSocket 支持以进行实时通信
5. 添加日志记录和错误处理
6. 支持配置文件自定义端口和其他设置