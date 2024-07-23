use serde::Serialize;
use axum::extract::ws::Message;
use serde_json::to_string;

#[derive(Clone, Serialize, Debug)]
pub enum ClientRequestType {
    LobbyUpdate
}

#[derive(Clone, Serialize, Debug)]
pub struct ClientRequest {
    req_type: ClientRequestType,
    content: String
}

impl ClientRequest {
    pub fn new(req_type: ClientRequestType, content: String) -> ClientRequest {
        ClientRequest {
            req_type, content
        }
    }

    pub fn to_msg(&self) -> Message {
        Message::Text(to_string(&self).unwrap_or_else(|_| { "Serialize error".to_string() }))
    }
}