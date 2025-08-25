use std::fmt::{Debug, Display};

use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;
use tungstenite::{Message, connect};

pub fn send_websocket_msg<T>(socket_address: &str, payload: T)
where
    T: Debug + Display + Serialize + DeserializeOwned,
{
    let (mut socket, response) = connect(socket_address).expect("Can't connect");

    log::info!("Connected to the server");
    ::log::info!("Response HTTP code: {}", response.status());

    log::info!("Response contains the following headers:");
    for (header, _value) in response.headers() {
        println!("* {header}");
    }

    let message = json!({"message":payload});
    if let Err(err) = socket.send(Message::Text(message.to_string().into())) {
        log::error!("failed to send {payload} due to {err}");
    } else {
        log::info!("message sent >>>>>>>.")
    }
}
