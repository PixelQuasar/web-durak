use axum::http::StatusCode;

pub fn error_message<E>(err: E) -> String
where
    E: std::error::Error,
{
    err.to_string()
}

pub fn error_msg_to_server_error(msg: String) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, msg)
}
