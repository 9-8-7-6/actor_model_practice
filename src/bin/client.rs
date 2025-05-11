// src/bin/client.rs
use actor_model_practice::{Message, Order};
use anyhow::Result;
use serde_json::to_string;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:12345").await?;
    let msg = Message {
        order: Order::BUY,
        ticker: "AAPL".into(),
        amount: 7.0,
    };

    let json = to_string(&msg)?;
    stream.write_all(json.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    println!("Sent order: {:?}", msg);

    Ok(())
}
