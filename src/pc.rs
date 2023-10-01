use std::net::TcpStream;

use websocket::{native_tls::TlsStream, sync::Client, ClientBuilder, OwnedMessage, WebSocketError};

enum WsClient {
    None,
    Insecure(Client<TcpStream>),
    Secure(Client<TlsStream<TcpStream>>),
}

pub struct WsConnection {
    client: WsClient,
    _url: String,
}

pub type WsChannnel = WsConnection;

pub fn ws_open_rust(url: String) -> Option<WsChannnel> {
    let secure = url.starts_with("wss");
    let builder = ClientBuilder::new("ws://127.0.0.1:7878");
    if builder.is_err() {
        return None;
    }
    let mut builder = builder.expect("Failed to expect none err");

    if secure {
        let connector = builder.connect_secure(None);
        if connector.is_err() {
            return Some(WsConnection {
                client: WsClient::None,
                _url: url,
            });
        }
        let client = connector.expect("Failed to expect none err");
        client.set_nonblocking(true).unwrap();
        return Some(WsConnection {
            client: WsClient::Secure(client),
            _url: url,
        });
    }
    let connector = builder.connect_insecure();
    if connector.is_err() {
        return Some(WsConnection {
            client: WsClient::None,
            _url: url,
        });
    }
    let client = connector.expect("Failed to expect none err");
    client.set_nonblocking(true).unwrap();
    return Some(WsConnection {
        client: WsClient::Insecure(client),
        _url: url,
    });
}

pub fn ws_write_raw(socket: &mut WsChannnel, data: OwnedMessage) -> bool {
    match &mut socket.client {
        WsClient::None => {
            return false;
        }
        WsClient::Insecure(client) => {
            return client.send_message(&data).is_ok();
        }
        WsClient::Secure(client) => {
            return client.send_message(&data).is_ok();
        }
    }
}

pub fn ws_write_rust(socket: &mut WsChannnel, data: Vec<u8>) -> bool {
    ws_write_raw(socket, OwnedMessage::Binary(data))
}

pub fn ws_read_rust(socket: &mut WsChannnel) -> Option<Vec<u8>> {
    let message: Result<OwnedMessage, WebSocketError> = match socket.client {
        WsClient::None => Err(WebSocketError::NoDataAvailable),
        WsClient::Insecure(ref mut client) => client.recv_message(),
        WsClient::Secure(ref mut client) => client.recv_message(),
    };
    match message {
        Ok(message) => match message {
            websocket::OwnedMessage::Text(_) => {}
            websocket::OwnedMessage::Binary(data) => {
                return Some(data);
            }
            websocket::OwnedMessage::Close(_) => {
                socket.client = WsClient::None;
            }
            websocket::OwnedMessage::Ping(ping) => {
                let message = OwnedMessage::Pong(ping);
                ws_write_raw(socket, message);
            }
            websocket::OwnedMessage::Pong(_data) => {}
        },
        Err(ref e) => match e {
            WebSocketError::IoError(err) => if err.kind() == std::io::ErrorKind::WouldBlock {},
            WebSocketError::NoDataAvailable => {
                socket.client = WsClient::None;
            }
            _ => {}
        },
    }
    return None;
}

pub fn ws_close_rust(socket: &mut WsChannnel) {
    match &socket.client {
        WsClient::None => {}
        WsClient::Insecure(client) => {
            client.shutdown().unwrap();
            socket.client = WsClient::None;
        }
        WsClient::Secure(client) => {
            client.shutdown().unwrap();
            socket.client = WsClient::None;
        }
    }
}

pub fn ws_state_rust(socket: &mut WsChannnel) -> i32{
    match &socket.client {
        WsClient::None => {
            2
        }
        WsClient::Insecure(_) => {
            1
        }
        WsClient::Secure(_) => {
            1
        }
    }
}