use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

mod web;

#[derive(Debug, Deserialize, Serialize)]
struct McpMessage {
    id: String,
    method: String,
    params: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct McpResponse {
    id: String,
    result: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
}

// 存储MCP消息的历史记录
type Messages = Arc<RwLock<Vec<String>>>;

async fn handle_mcp_request(message: McpMessage, messages: Messages) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Received MCP message: {:?}", message);
    
    // 记录消息到历史
    let message_str = format!("{:?}", message);
    {
        let mut msgs = messages.write().await;
        msgs.push(message_str);
        // 限制历史记录数量为100条
        if msgs.len() > 100 {
            msgs.remove(0);
        }
    }
    
    // 构造响应
    let response = McpResponse {
        id: message.id,
        result: Some(serde_json::json!({"status": "success"})),
        error: None,
    };
    
    Ok(warp::reply::json(&response))
}

#[tokio::main]
async fn main() {
    println!("Starting MCP Server...");
    
    // 初始化消息存储
    let messages: Messages = Arc::new(RwLock::new(Vec::new()));
    
    // 克隆消息存储以在不同路由中使用
    let messages_clone = messages.clone();
    
    // 添加CORS头部
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);
    
    // 定义MCP端点
    let mcp_route = warp::path("mcp")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_messages(messages))
        .and_then(handle_mcp_request);
    
    // 健康检查端点
    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));
    
    // API路由
    let api_routes = mcp_route.or(health_route)
        .with(cors); // 应用CORS到API路由
    
    // 启动Web服务器
    tokio::spawn(async move {
        web::serve_web_interface(messages_clone).await;
    });
    
    // 启动API服务器（在3032端口）
    println!("API server listening on http://localhost:3032");
    warp::serve(api_routes)
        .run(([0, 0, 0, 0], 3032))
        .await;
}

// 依赖注入辅助函数
fn with_messages(messages: Messages) -> impl Filter<Extract = (Messages,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || messages.clone())
}