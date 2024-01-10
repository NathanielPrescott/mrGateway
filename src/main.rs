use actix_web::{App, get, HttpServer, Responder};

#[get("/traversal")]
async fn traversal() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let domain = "localhost";
    let port = 8080;

    println!("Starting server...");
    println!("Listening on: http://{}:{}", domain, port);

    HttpServer::new(move || {
        App::new().service(traversal)
    }).bind((domain, port))?.run().await
}
