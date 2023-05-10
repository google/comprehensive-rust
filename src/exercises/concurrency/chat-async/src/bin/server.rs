use futures_util::sink::SinkExt;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_websockets::{ServerBuilder, WebsocketStream, Message};

#[derive(Error, Debug)]
enum ServerError {
    #[error("websocket error: {0}")]
    Websocket(String),
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
}

// tokio_websockets Error types do not implement std::error::Error, so we make do by just capturing
// the debug format for the error.
impl From<tokio_websockets::Error> for ServerError {
    fn from(err: tokio_websockets::Error) -> Self {
        ServerError::Websocket(format!("{:?}", err))
    }
}

impl From<tokio_websockets::proto::ProtocolError> for ServerError {
    fn from(err: tokio_websockets::proto::ProtocolError) -> Self {
        ServerError::Websocket(format!("{:?}", err))
    }
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebsocketStream<TcpStream>,
) -> Result<(), ServerError> {
    ws_stream.send(Message::text("Welcome to chat! Type a message".into())).await?;
    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        println!("From client {addr:?} {:?}", msg.as_text()?);
                        // TODO: broadcast the message to other connected clients
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()),
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream).await
        });
    }
}
