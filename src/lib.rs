use serde::{Deserialize, Serialize};

/// Represents the type of order in the trading system.
#[derive(Serialize, Deserialize, Debug)]
pub enum Order {
    BUY,
    SELL,
}

/// A message sent to the actor for processing an order.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub order: Order,
    pub ticker: String,
    pub amount: f32,
}
