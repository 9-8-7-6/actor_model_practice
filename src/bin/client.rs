use actor_model_practice::{Message, Order};
use tokio::main;
use tokio::sync::{mpsc::Sender, oneshot};

/// Sends a BUY order to the actor via the given sender channel.
async fn send_order(sender: Sender<Message>, amount: f32, ticker: &str) {
    let (responder, receiver) = oneshot::channel();
    let msg = Message {
        order: Order::BUY,
        ticker: ticker.to_string(),
        amount,
        respond_to: responder,
    };

    let _ = sender.send(msg).await;

    match receiver.await {
        Ok(result) => println!("Order result for {}: {}", ticker, result),
        Err(e) => eprintln!("Failed to receive response: {}", e),
    }
}

#[main]
async fn main() {
    let (tx, _rx) = tokio::sync::mpsc::channel::<Message>(32);

    for t in ["TSLA", "AAPL", "PLTR"] {
        send_order(tx.clone(), 7.0, t).await;
    }
}
