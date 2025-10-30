use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateOrderResponse {
    pub order_id: String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DeletOrderResponse {
    pub filled_qty: u32,
    pub average_price: u32
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DepthResponse {
    pub bids: Vec<[u32; 2]>,
    pub asks: Vec<[u32; 2]>,
    pub lastUpdateId: String
}