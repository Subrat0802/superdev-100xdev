
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq)]
pub enum Side {
    Buy, 
    Sell
}

#[derive(Deserialize, Debug)]
pub struct CreateOrderinput {
    pub price: u32,
    pub quantity: u32,
    pub user_id: u32,
    pub side: Side
}

#[derive(Deserialize, Serialize)]
pub struct DeleteOrder {
    pub order_id: String
}