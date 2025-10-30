use actix_web::{HttpResponse, Responder, delete, get, post, web::Json};

use crate::{inputs::{CreateOrderinput, DeleteOrder}, outputs::{CreateOrderResponse, DeletOrderResponse, DepthResponse}};


#[post("/order")]
pub async fn create_order(body: Json<CreateOrderinput>) -> impl Responder{
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: String::from("ads")
    })
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrder>) -> impl Responder{
    let order_id = body.order_id;
    HttpResponse::Ok().json(DeletOrderResponse{
        filled_qty: 0,
        average_price: 100
    })

}

#[get("/depth")]
pub async fn get_depth() -> impl Responder{
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
