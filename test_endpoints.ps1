# MCP Server 端点测试脚本

Write-Host "Testing MCP Server endpoints..." -ForegroundColor Green

# 测试健康检查端点 (GET)
Write-Host "`n1. Testing health endpoint (GET http://localhost:3032/health)" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://localhost:3032/health" -Method GET
    Write-Host "Status: $($response.StatusCode)" -ForegroundColor Green
    Write-Host "Content: $($response.Content)" -ForegroundColor Cyan
} catch {
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
}

# 测试MCP消息端点 (POST)
Write-Host "`n2. Testing MCP message endpoint (POST http://localhost:3032/mcp)" -ForegroundColor Yellow
try {
    $body = @{
        id = "test-1"
        method = "echo"
        params = @{
            message = "Hello MCP Server!"
        }
    } | ConvertTo-Json
    
    $response = Invoke-WebRequest -Uri "http://localhost:3032/mcp" -Method POST -Body $body -ContentType "application/json"
    Write-Host "Status: $($response.StatusCode)" -ForegroundColor Green
    Write-Host "Content: $($response.Content)" -ForegroundColor Cyan
} catch {
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
}

# 测试错误的HTTP方法
Write-Host "`n3. Testing wrong HTTP method (GET on /mcp endpoint)" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://localhost:3032/mcp" -Method GET
    Write-Host "Status: $($response.StatusCode)" -ForegroundColor Green
    Write-Host "Content: $($response.Content)" -ForegroundColor Cyan
} catch {
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
    # 这里应该会显示 "HTTP method not allowed" 错误
}

# 测试消息历史API (GET)
Write-Host "`n4. Testing messages history endpoint (GET http://localhost:3030/api/messages)" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://localhost:3030/api/messages" -Method GET
    Write-Host "Status: $($response.StatusCode)" -ForegroundColor Green
    Write-Host "Content: $($response.Content)" -ForegroundColor Cyan
} catch {
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
}

# 测试清除消息API (POST)
Write-Host "`n5. Testing clear messages endpoint (POST http://localhost:3030/api/clear)" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://localhost:3030/api/clear" -Method POST
    Write-Host "Status: $($response.StatusCode)" -ForegroundColor Green
    Write-Host "Content: $($response.Content)" -ForegroundColor Cyan
} catch {
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`nEndpoint testing completed." -ForegroundColor Green