# MCP Server API 文档

## 端点概览

MCP服务器提供两个不同的服务端口：
- Web管理界面: [http://localhost:3030](http://localhost:3030)
- API服务: [http://localhost:3032](http://localhost:3032)

## CORS支持

为了解决跨域问题，服务器已配置CORS支持，允许从任何源访问API端点。

## API端点详情

### 1. 健康检查端点
**URL**: [http://localhost:3032/health](http://localhost:3032/health)  
**方法**: `GET`  
**描述**: 检查服务器是否正常运行  
**响应**: 
```json
{
  "status": "ok"
}
```

**示例请求**:
```powershell
Invoke-WebRequest -Uri "http://localhost:3032/health" -Method GET
```

### 2. MCP消息处理端点
**URL**: [http://localhost:3032/mcp](http://localhost:3032/mcp)  
**方法**: `POST`  
**描述**: 发送MCP协议消息到服务器  
**请求体**: JSON格式的MCP消息
```json
{
  "id": "string",
  "method": "string",
  "params": {
    // 可选参数
  }
}
```

**响应**:
```json
{
  "id": "string",
  "result": {
    "status": "success"
  },
  "error": null
}
```

**示例请求**:
```powershell
$body = @{
    id = "1"
    method = "echo"
    params = @{
        message = "Hello MCP Server!"
    }
} | ConvertTo-Json

Invoke-WebRequest -Uri "http://localhost:3032/mcp" -Method POST -Body $body -ContentType "application/json"
```

### 3. 消息历史获取端点
**URL**: [http://localhost:3030/api/messages](http://localhost:3030/api/messages)  
**方法**: `GET`  
**描述**: 获取已接收的MCP消息历史记录  
**响应**:
```json
{
  "messages": [
    "McpMessage { id: \"1\", method: \"echo\", params: Some(Object {\"message\": String(\"Hello MCP Server!\")}) }"
  ]
}
```

**示例请求**:
```powershell
Invoke-WebRequest -Uri "http://localhost:3030/api/messages" -Method GET
```

### 4. 清除消息历史端点
**URL**: [http://localhost:3030/api/clear](http://localhost:3030/api/clear)  
**方法**: `POST`  
**描述**: 清除所有已记录的MCP消息历史  
**响应**:
```json
{
  "status": "cleared"
}
```

**示例请求**:
```powershell
Invoke-WebRequest -Uri "http://localhost:3030/api/clear" -Method POST
```

## 常见错误及解决方案

### HTTP 405: Method Not Allowed
**原因**: 使用了不正确的HTTP方法访问端点

**解决方案**:
1. 确保使用正确的HTTP方法：
   - GET用于获取数据（健康检查、消息历史）
   - POST用于发送数据（MCP消息、清除历史）

2. 检查URL是否正确：
   - API服务在3032端口
   - Web界面在3030端口

### TypeError: Failed to fetch (Web界面)
**原因**: 浏览器的CORS策略限制或跨域问题

**解决方案**:
1. 确保服务器已正确配置CORS支持（已实现）
2. 使用支持的浏览器访问Web界面
3. 检查网络连接是否正常

### 连接被拒绝
**原因**: 服务器未启动或端口被占用

**解决方案**:
1. 确认服务器正在运行
2. 检查端口是否被其他程序占用
3. 重启服务器

## PowerShell测试脚本

使用项目中的`test_endpoints.ps1`脚本可以快速验证所有端点是否正常工作：
```powershell
powershell -ExecutionPolicy Bypass -File test_endpoints.ps1
```

## Web界面使用

通过浏览器访问 [http://localhost:3030](http://localhost:3030) 可以使用Web管理界面：
1. 实时查看服务器状态
2. 发送测试MCP消息
3. 查看消息历史记录
4. 清除历史记录

Web界面已解决跨域问题，可以直接在页面上发送消息并查看响应。所有API端点都以超链接形式显示，方便直接访问测试。