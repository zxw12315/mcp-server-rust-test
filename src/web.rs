use warp::Filter;
use std::sync::Arc;
use tokio::sync::RwLock;

// 存储MCP消息的历史记录
type Messages = Arc<RwLock<Vec<String>>>;

pub async fn serve_web_interface(messages: Messages) {
    // 静态文件服务
    let static_files = warp::path("static")
        .and(warp::fs::dir("static/"));
    
    // 主页路由
    let index = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::html(INDEX_HTML));
    
    // API路由 - 获取消息历史
    let get_messages = warp::path("api")
        .and(warp::path("messages"))
        .and(warp::path::end())
        .and(with_messages(messages.clone()))
        .and(warp::get())
        .and_then(get_messages_handler);
    
    // API路由 - 清除消息历史
    let clear_messages = warp::path("api")
        .and(warp::path("clear"))
        .and(warp::path::end())
        .and(with_messages(messages.clone()))
        .and(warp::post())
        .and_then(clear_messages_handler);
    
    // 添加CORS头部到所有响应
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);
    
    let routes = index.or(static_files).or(get_messages).or(clear_messages)
        .with(cors); // 应用CORS到所有路由
    
    println!("Web interface listening on http://localhost:3030");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

// 处理获取消息的请求
async fn get_messages_handler(messages: Messages) -> Result<impl warp::Reply, warp::Rejection> {
    let msgs = messages.read().await;
    let json = serde_json::json!({
        "messages": &*msgs
    });
    Ok(warp::reply::json(&json))
}

// 处理清除消息的请求
async fn clear_messages_handler(messages: Messages) -> Result<impl warp::Reply, warp::Rejection> {
    let mut msgs = messages.write().await;
    msgs.clear();
    let json = serde_json::json!({
        "status": "cleared"
    });
    Ok(warp::reply::json(&json))
}

// 依赖注入辅助函数
fn with_messages(messages: Messages) -> impl Filter<Extract = (Messages,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || messages.clone())
}

// 主页HTML
const INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>MCP Server</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .container {
            background-color: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        h1 {
            color: #333;
            text-align: center;
        }
        h2 {
            color: #555;
            border-bottom: 1px solid #eee;
            padding-bottom: 10px;
        }
        .status {
            text-align: center;
            padding: 10px;
            margin: 20px 0;
            border-radius: 4px;
            background-color: #e8f5e9;
            color: #2e7d32;
        }
        .message-list {
            border: 1px solid #ddd;
            border-radius: 4px;
            padding: 10px;
            height: 300px;
            overflow-y: auto;
            margin: 20px 0;
            background-color: #fafafa;
        }
        .message-item {
            padding: 8px;
            border-bottom: 1px solid #eee;
            word-break: break-all;
        }
        .message-item:last-child {
            border-bottom: none;
        }
        .controls {
            text-align: center;
            margin: 20px 0;
        }
        button {
            background-color: #2196f3;
            color: white;
            border: none;
            padding: 10px 20px;
            margin: 0 10px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 16px;
        }
        button:hover {
            background-color: #1976d2;
        }
        button.clear {
            background-color: #f44336;
        }
        button.clear:hover {
            background-color: #d32f2f;
        }
        .endpoint-info {
            background-color: #e3f2fd;
            padding: 15px;
            border-radius: 4px;
            margin: 20px 0;
        }
        .endpoint-info h3 {
            margin-top: 0;
        }
        .endpoint-info a {
            display: inline-block;
            margin: 5px 0;
            padding: 5px 10px;
            background-color: #d1c4e9;
            color: #4527a0;
            text-decoration: none;
            border-radius: 3px;
            font-family: monospace;
        }
        .endpoint-info a:hover {
            background-color: #b39ddb;
        }
        .test-section {
            background-color: #fff3e0;
            padding: 15px;
            border-radius: 4px;
            margin: 20px 0;
        }
        .test-section textarea {
            width: 100%;
            height: 100px;
            margin: 10px 0;
            padding: 8px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-family: monospace;
        }
        .test-section button {
            background-color: #4caf50;
        }
        .test-section button:hover {
            background-color: #388e3c;
        }
        .error {
            color: #f44336;
            background-color: #ffebee;
            padding: 10px;
            border-radius: 4px;
            margin: 10px 0;
            display: none;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>MCP Server 控制面板</h1>
        
        <div class="status" id="status">
            连接中...
        </div>
        
        <div class="endpoint-info">
            <h3>API 端点</h3>
            <p><strong>MCP 消息端点:</strong> <a href="http://localhost:3032/mcp" target="_blank">POST http://localhost:3032/mcp</a></p>
            <p><strong>健康检查端点:</strong> <a href="http://localhost:3032/health" target="_blank">GET http://localhost:3032/health</a></p>
            <p><strong>Web 界面端点:</strong> <a href="http://localhost:3030/" target="_blank">GET http://localhost:3030/</a></p>
            <p><strong>消息历史端点:</strong> <a href="http://localhost:3030/api/messages" target="_blank">GET http://localhost:3030/api/messages</a></p>
            <p><strong>清除历史端点:</strong> <a href="http://localhost:3030/api/clear" target="_blank">POST http://localhost:3030/api/clear</a></p>
        </div>
        
        <div class="test-section">
            <h3>测试 MCP 消息</h3>
            <textarea id="messageInput">{
  "id": "1",
  "method": "echo",
  "params": {
    "message": "Hello MCP Server!"
  }
}</textarea>
            <button onclick="sendMessage()">发送消息</button>
            <div id="errorArea" class="error"></div>
            <div id="responseArea" style="margin-top: 10px; padding: 10px; background-color: #f0f0f0; border-radius: 4px; display: none;"></div>
        </div>
        
        <div class="controls">
            <button onclick="refreshMessages()">刷新消息</button>
            <button class="clear" onclick="clearMessages()">清除历史</button>
        </div>
        
        <h2>消息历史</h2>
        <div class="message-list" id="messageList">
            <div class="message-item">暂无消息</div>
        </div>
    </div>

    <script>
        // 页面加载时获取消息
        window.onload = function() {
            updateStatus();
            refreshMessages();
            // 每5秒自动刷新一次
            setInterval(refreshMessages, 5000);
        };
        
        // 更新连接状态
        function updateStatus() {
            fetch('http://localhost:3032/health')
                .then(response => response.json())
                .then(data => {
                    const statusEl = document.getElementById('status');
                    if (data.status === 'ok') {
                        statusEl.textContent = '服务器运行正常';
                        statusEl.style.backgroundColor = '#e8f5e9';
                        statusEl.style.color = '#2e7d32';
                    } else {
                        statusEl.textContent = '服务器连接失败';
                        statusEl.style.backgroundColor = '#ffebee';
                        statusEl.style.color = '#c62828';
                    }
                })
                .catch(error => {
                    const statusEl = document.getElementById('status');
                    statusEl.textContent = '服务器连接失败';
                    statusEl.style.backgroundColor = '#ffebee';
                    statusEl.style.color = '#c62828';
                });
        }
        
        // 刷新消息列表
        function refreshMessages() {
            fetch('/api/messages')
                .then(response => response.json())
                .then(data => {
                    const messageList = document.getElementById('messageList');
                    if (data.messages.length === 0) {
                        messageList.innerHTML = '<div class="message-item">暂无消息</div>';
                    } else {
                        messageList.innerHTML = '';
                        // 反向显示，最新的在上面
                        for (let i = data.messages.length - 1; i >= 0; i--) {
                            const messageDiv = document.createElement('div');
                            messageDiv.className = 'message-item';
                            messageDiv.textContent = data.messages[i];
                            messageList.appendChild(messageDiv);
                        }
                    }
                })
                .catch(error => {
                    console.error('Error fetching messages:', error);
                });
        }
        
        // 清除消息历史
        function clearMessages() {
            fetch('/api/clear', { method: 'POST' })
                .then(response => response.json())
                .then(data => {
                    if (data.status === 'cleared') {
                        refreshMessages();
                    }
                })
                .catch(error => {
                    console.error('Error clearing messages:', error);
                });
        }
        
        // 发送测试消息
        function sendMessage() {
            const messageInput = document.getElementById('messageInput');
            const responseArea = document.getElementById('responseArea');
            const errorArea = document.getElementById('errorArea');
            
            // 隐藏之前的错误信息
            errorArea.style.display = 'none';
            responseArea.style.display = 'none';
            
            try {
                const message = JSON.parse(messageInput.value);
                
                fetch('http://localhost:3032/mcp', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(message)
                })
                .then(response => {
                    if (!response.ok) {
                        throw new Error(`HTTP error! status: ${response.status}`);
                    }
                    return response.json();
                })
                .then(data => {
                    responseArea.style.display = 'block';
                    responseArea.innerHTML = '<strong>响应:</strong><br>' + JSON.stringify(data, null, 2);
                    refreshMessages(); // 刷新消息列表
                })
                .catch(error => {
                    errorArea.style.display = 'block';
                    errorArea.innerHTML = '<strong>错误:</strong><br>' + error.toString();
                    console.error('Error sending message:', error);
                });
            } catch (e) {
                errorArea.style.display = 'block';
                errorArea.innerHTML = '<strong>错误:</strong><br>无效的JSON格式: ' + e.message;
            }
        }
    </script>
</body>
</html>
"#;