use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let mut ws_stream = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
        .connect()
        .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin);

    loop {
        let mut line = String::new();
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => println!("From server: {}", msg.as_text()?),
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()),
                }
            }
            res = stdin.read_line(&mut line) => {
                match res {
                    Ok(0) => return Ok(()),
                    Ok(_) => ws_stream.send(Message::text(line.trim_end().to_string())).await?,
                    Err(err) => return Err(err.into()),
                }
            }

        }
    }
}
