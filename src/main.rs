mod server;
mod database;

#[actix_web::main]
async fn main() {
    server::run().await.unwrap();
}
