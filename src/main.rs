mod solver;
mod server;
mod database;

#[actix_web::main]
async fn main() {
    database::connect().ok();
    server::run().await.unwrap();
}
