use actix::{Actor, StreamHandler, Handler, Message as ActixMessage, AsyncContext, ActorContext, Addr};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use log::{info, warn, error};

/// WebSocket 消息类型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum WSMessage {
    #[serde(rename = "command")]
    Command {
        target_client: Uuid,
        request_id: String,
        command: String,
        params: serde_json::Value,
    },
    #[serde(rename = "response")]
    Response {
        request_id: String,
        success: bool,
        data: serde_json::Value,
    },
    #[serde(rename = "heartbeat")]
    Heartbeat,
    #[serde(rename = "register")]
    Register {
        client_uuid: Uuid,
        client_type: String, // "client" 或 "lms"
    },
}

/// WebSocket 连接管理器
#[derive(Clone)]
pub struct WSConnectionManager {
    // client_uuid -> WebSocket Actor Address
    connections: Arc<Mutex<HashMap<Uuid, Addr<WSConnection>>>>,
}

impl WSConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, uuid: Uuid, addr: Addr<WSConnection>) {
        self.connections.lock().unwrap().insert(uuid, addr);
        info!("WebSocket registered: {}", uuid);
    }

    pub fn unregister(&self, uuid: &Uuid) {
        self.connections.lock().unwrap().remove(uuid);
        info!("WebSocket unregistered: {}", uuid);
    }

    pub fn send_to_client(&self, uuid: Uuid, msg: WSMessage) -> Result<(), String> {
        if let Some(addr) = self.connections.lock().unwrap().get(&uuid) {
            addr.do_send(SendWSMessage(msg));
            Ok(())
        } else {
            Err(format!("Client {} not connected", uuid))
        }
    }

    pub fn get_online_count(&self) -> usize {
        self.connections.lock().unwrap().len()
    }

    pub fn get_online_clients(&self) -> Vec<Uuid> {
        self.connections.lock().unwrap().keys().copied().collect()
    }
}

/// WebSocket Actor
pub struct WSConnection {
    uuid: Option<Uuid>,
    client_type: Option<String>,
    manager: web::Data<WSConnectionManager>,
}

impl WSConnection {
    pub fn new(manager: web::Data<WSConnectionManager>) -> Self {
        Self {
            uuid: None,
            client_type: None,
            manager,
        }
    }
}

impl Actor for WSConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("WebSocket connection started");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        if let Some(uuid) = self.uuid {
            self.manager.unregister(&uuid);
            info!("WebSocket disconnected: {}", uuid);
        }
    }
}

/// 处理 WebSocket 消息
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => {}
            Ok(ws::Message::Text(text)) => {
                // 解析消息
                match serde_json::from_str::<WSMessage>(&text) {
                    Ok(ws_msg) => self.handle_ws_message(ws_msg, ctx),
                    Err(e) => {
                        error!("Failed to parse WebSocket message: {}", e);
                    }
                }
            }
            Ok(ws::Message::Binary(_)) => {
                warn!("Binary messages not supported");
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl WSConnection {
    fn handle_ws_message(&mut self, msg: WSMessage, ctx: &mut ws::WebsocketContext<Self>) {
        match msg {
            WSMessage::Register { client_uuid, client_type } => {
                self.uuid = Some(client_uuid);
                self.client_type = Some(client_type.clone());
                self.manager.register(client_uuid, ctx.address());

                info!("Client registered: {} (type: {})", client_uuid, client_type);

                // 发送确认
                let response = WSMessage::Response {
                    request_id: "register".to_string(),
                    success: true,
                    data: serde_json::json!({"message": "Registered successfully"}),
                };
                if let Ok(json) = serde_json::to_string(&response) {
                    ctx.text(json);
                }
            }
            WSMessage::Heartbeat => {
                let response = WSMessage::Response {
                    request_id: "heartbeat".to_string(),
                    success: true,
                    data: serde_json::json!({"timestamp": chrono::Utc::now()}),
                };
                if let Ok(json) = serde_json::to_string(&response) {
                    ctx.text(json);
                }
            }
            WSMessage::Response { .. } => {
                // 响应消息会被路由到等待的请求
                // 这里可以实现请求-响应映射逻辑
                if let Ok(json) = serde_json::to_string(&msg) {
                    ctx.text(json);
                }
            }
            WSMessage::Command { .. } => {
                // 从客户端接收的命令响应
                if let Ok(json) = serde_json::to_string(&msg) {
                    ctx.text(json);
                }
            }
        }
    }
}

/// 内部消息：发送 WebSocket 消息
#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct SendWSMessage(pub WSMessage);

impl Handler<SendWSMessage> for WSConnection {
    type Result = ();

    fn handle(&mut self, msg: SendWSMessage, ctx: &mut Self::Context) {
        if let Ok(json) = serde_json::to_string(&msg.0) {
            ctx.text(json);
        }
    }
}

/// WebSocket 端点
pub async fn ws_endpoint(
    req: HttpRequest,
    stream: web::Payload,
    manager: web::Data<WSConnectionManager>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WSConnection::new(manager),
        &req,
        stream,
    )
}

/// HTTP API：向客户端发送命令
#[derive(Debug, Deserialize)]
pub struct SendCommandRequest {
    pub target_client: Uuid,
    pub request_id: String,
    pub command: String,
    pub params: serde_json::Value,
}

pub async fn send_command(
    manager: web::Data<WSConnectionManager>,
    req: web::Json<SendCommandRequest>,
) -> Result<HttpResponse, Error> {
    let msg = WSMessage::Command {
        target_client: req.target_client,
        request_id: req.request_id.clone(),
        command: req.command.clone(),
        params: req.params.clone(),
    };

    match manager.send_to_client(req.target_client, msg) {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Command sent"
        }))),
        Err(e) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": e
        }))),
    }
}

/// 获取在线连接状态
pub async fn get_connections_status(
    manager: web::Data<WSConnectionManager>,
) -> Result<HttpResponse, Error> {
    let online_count = manager.get_online_count();
    let online_clients = manager.get_online_clients();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "online_count": online_count,
            "online_clients": online_clients
        }
    })))
}
