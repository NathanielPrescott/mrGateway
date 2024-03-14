use actix_web::{get, HttpRequest, Responder};

// Generate documentation for the following functions

#[get("/is_alive")]
pub async fn is_alive() -> impl Responder {
    println!("Log: is_alive() called");
    "Gateway is alive!"
}

#[get("/pass_params")]
pub async fn pass_params(req: HttpRequest) -> impl Responder {
    println!("Log: pass_params() called");
    let query = req.query_string();
    format!("Query: {}", query)
}