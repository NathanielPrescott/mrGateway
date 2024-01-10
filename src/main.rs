use actix_cors::Cors;
use actix_web::{App, get, HttpServer, Responder};

#[get("/traversal")]
async fn traversal() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let domain = "0.0.0.0";
    let port = 8080;

    println!("Starting server...");
    println!("Listening on: http://{}:{}", domain, port);

    HttpServer::new(move || {
        App::new().wrap(Cors::permissive()).service(traversal)
    }).bind((domain, port))?.run().await
}
