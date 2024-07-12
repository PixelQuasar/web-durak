pub async fn message_body_handler (msg: &str) {
    let parsed_array: Vec<&str> = msg.split("#").collect();

    if parsed_array.len() < 1 {
        return;
    }

    let command = parsed_array[0];

    match command {
        "JOIN" =>
    }
}
