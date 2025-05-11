use tokio::sync::oneshot;

/// Represents the type of order in the trading system.
#[derive(Debug, Clone)]
pub enum Order {
    BUY,
    SELL,
}

/// A message sent to the actor for processing an order.
#[derive(Debug)]
pub struct Message {
    pub order: Order,
    pub ticker: String,
    pub amount: f32,
    pub respond_to: oneshot::Sender<i32>,
}
