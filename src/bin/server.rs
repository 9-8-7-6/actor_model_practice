use actor_model_practice::{Message, Order};
use tokio::main;
use tokio::sync::mpsc;

/// Represents the server-side actor that processes incoming orders.
pub struct OrderBookActor {
    /// Channel to receive messages from clients.
    pub receiver: mpsc::Receiver<Message>,
    /// The total amount currently invested.
    pub total_invested: f32,
    /// The maximum allowed investment cap.
    pub investment_cap: f32,
}

impl OrderBookActor {
    /// Constructs a new OrderBookActor with a message receiver and investment cap.
    fn new(receiver: mpsc::Receiver<Message>, investment_cap: f32) -> Self {
        Self {
            receiver,
            total_invested: 0.0,
            investment_cap,
        }
    }

    /// Handles a single incoming message, updates state, and sends back a response.
    fn handle_message(&mut self, message: Message) {
        if message.amount + self.total_invested >= self.investment_cap {
            println!(
                "rejecting purchase, total invested: {}",
                self.total_invested
            );
            let _ = message.respond_to.send(0);
        } else {
            self.total_invested += message.amount;
            println!(
                "processing purchase, total invested: {}",
                self.total_invested
            );
            let _ = message.respond_to.send(1);
        }
    }
    /// Main actor loop that continuously receives and processes messages.
    async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}

#[main]
async fn main() {
    // Create a channel for communication with clients.
    let (tx, rx) = mpsc::channel::<Message>(32);

    /* for test client function
    tokio::spawn(async move {
        use tokio::sync::oneshot;

        for _ in 0..3 {
            let (responder, receiver) = oneshot::channel();
            let msg = Message {
                order: Order::BUY,
                ticker: "BYND".to_string(),
                amount: 5.5,
                respond_to: responder,
            };

            let _ = tx.send(msg);
            if let Ok(v) = receiver.await {
                println!("Client got response: {}", v);
            }
        }
    });
    */

    let actor = OrderBookActor::new(rx, 20.0);
    actor.run().await;
}
