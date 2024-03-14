
mod network;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    network::external_connection().await
}
