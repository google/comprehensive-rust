use futures_util::sink::SinkExt;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::error::{RecvError, SendError};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebsocketStream};

#[derive(Error, Debug)]
enum ServerError {
    #[error("websocket error: {0}")]
    Websocket(String),
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("broadcast channel SendError: {0}")]
    SendError(#[from] SendError<String>),
    #[error("broadcast channel RecvError: {0}")]
    RecvError(#[from] RecvError),
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
    bcast_tx: Sender<String>,
) -> Result<(), ServerError> {
    ws_stream
        .send(Message::text("Welcome to chat! Type a message".into()))
        .await?;
    let mut bcast_rx = bcast_tx.subscribe();
    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        let msg = msg.as_text()?;
                        println!("From client {addr:?} {msg:?}");
                        bcast_tx.send(msg.into())?;
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()),
                }
            }
            msg = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg?)).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}
