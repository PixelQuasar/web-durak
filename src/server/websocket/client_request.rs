use serde::Serialize;
use serde_json::to_string;

#[derive(Clone, Serialize, Debug)]
pub enum ClientRequestType {
    LobbyUpdate,
    GameCreate,
    GameUpdate,
    GameFinish,
    GameDelete,
    Error
}

#[derive(Clone, Serialize, Debug)]
pub struct ClientRequest {
    req_type: ClientRequestType,
    content: String,
}

impl ClientRequest {
    pub fn new(req_type: ClientRequestType, content: String) -> ClientRequest {
        ClientRequest { req_type, content }
    }

    pub fn build_error(msg: String) -> ClientRequest {
        ClientRequest {
            req_type: ClientRequestType::Error,
            content: msg,
        }
    }

    pub fn to_string(&self) -> String {
        to_string(&self).unwrap_or_else(|_| "Serialize error".to_string())
    }
}
