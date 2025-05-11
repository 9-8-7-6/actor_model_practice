use actor_model_practice::{Message, Order};
use tokio::main;
use tokio::sync::{mpsc::Sender, oneshot};

async fn send_order(sender: Sender<Message>, amount: f32, ticker: &str) {
    let (responder, receiver) = oneshot::channel();
    let msg = Message {
        order: Order::BUY,
        ticker: ticker.to_string(),
        amount,
        respond_to: responder,
    };
    let _ = sender.send(msg).await;
    if let Ok(result) = receiver.await {
        println!("Order result: {}", result);
    }
}

#[main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(32);

    for t in ["TSLA", "AAPL", "PLTR"] {
        send_order(tx.clone(), 7.0, t).await;
    }

    tokio::spawn(async move {
        let mut total = 0.0;
        while let Some(msg) = rx.recv().await {
            total += msg.amount;
            let _ = msg.respond_to.send(1);
            println!("[server mock] processed {} total = {}", msg.ticker, total);
        }
    });
}
