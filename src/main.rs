mod solver;
mod server;
mod database;

#[actix_web::main]
async fn main() {
    database::connect();
    server::run().await.unwrap();
}
