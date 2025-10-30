use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, delete, get, post, web::{self, Json, Data}};

use crate::{inputs::{CreateOrderinput, DeleteOrder}, orderbook::{self, OrderBook}, outputs::{CreateOrderResponse, DeletOrderResponse, DepthResponse}};


#[post("/order")]
pub async fn create_order(body: Json<CreateOrderinput>, orderbook: Data<Arc<Mutex<OrderBook>>>) -> impl Responder{
    
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    let mut  orderbook = orderbook.lock().unwrap();
    orderbook.create_order(price, quantity, user_id, side);

    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: String::from("ads")
    })
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrder>, orderbook: Data<OrderBook>) -> impl Responder{
    let order_id = body.order_id;
    HttpResponse::Ok().json(DeletOrderResponse{
        filled_qty: 0,
        average_price: 100
    })

}

#[get("/depth")]
pub async fn get_depth(orderbook: Data<OrderBook>) -> impl Responder{
    // let depth = orderbook.get_depth();
    HttpResponse::Ok().json(DepthResponse {
        bids: vec![],
        asks: vec![],
        lastUpdateId: String::from("asdasd")
    })
}




// #[derive(Serialize, Deserialize)]
// struct Rect {
//     width: u32,
//     heigth: u32
// }


// println!("{:?}", body);
// let r = Rect {
//     width: 20,
//     heigth: 30
// };

// return HttpResponse::Ok().json(r);
