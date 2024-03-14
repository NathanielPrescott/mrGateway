use actix_web::{get, HttpRequest, Responder};

#[get("/is_alive")]
pub async fn is_alive() -> impl Responder {
    println!("Log: is_alive() called");
    "Gateway is alive!"
}

#[get("/doors_of_durin")]
pub async fn doors_of_durin(req: HttpRequest) -> impl Responder {
    println!("Log: doors_of_durin() called");
    let query_params = req.query_string();
    
    println!("Query: {}", query_params);
    format!("Query: {}", query_params)
}