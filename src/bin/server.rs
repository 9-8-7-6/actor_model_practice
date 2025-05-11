// src/bin/server.rs
use actor_model_practice::Message;
use anyhow::Result;
use serde_json::from_str;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
    sync::mpsc,
};

pub struct OrderBookActor {
    receiver: mpsc::Receiver<Message>,
    total_invested: f32,
    investment_cap: f32,
}

impl OrderBookActor {
    fn new(receiver: mpsc::Receiver<Message>, cap: f32) -> Self {
        Self {
            receiver,
            total_invested: 0.0,
            investment_cap: cap,
        }
    }

    fn handle_message(&mut self, msg: Message) {
        if self.total_invested + msg.amount > self.investment_cap {
            println!(
                "→ REJECT {:?} {:.2}, total {:.2}",
                msg.ticker, msg.amount, self.total_invested
            );
        } else {
            self.total_invested += msg.amount;
            println!(
                "✔ ACCEPT {:?} {:.2}, total now {:.2}",
                msg.ticker, msg.amount, self.total_invested
            );
        }
    }

    async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(async move { OrderBookActor::new(rx, 20.0).run().await });

    let listener = TcpListener::bind("127.0.0.1:12345").await?;
    println!("Server listening on 127.0.0.1:12345");

    loop {
        let (socket, _) = listener.accept().await?;
        let tx = tx.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(socket);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                match from_str::<Message>(&line) {
                    Ok(msg) => {
                        let _ = tx.send(msg).await;
                    }
                    Err(e) => eprintln!("bad json: {}", e),
                }
            }
        });
    }
}
