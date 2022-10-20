use actix_web::{web, dev, get, HttpServer, App, HttpResponse, Responder};
use std::error::Error;
use std::sync::Mutex;

use super::database::model::Collection;

mod responders;
mod routes;

pub struct ServerState {
	collections: Mutex<Vec<Collection>>,
}

pub struct Server;

#[get("/")]
async fn get_home() -> impl Responder {
	HttpResponse::Ok().body("
		<h4>If you are seeing this message, the server is running. Sample as dat :)</h4>
	")
}

impl Server {
	pub async fn new(state: web::Data<ServerState>) -> Result<dev::Server, Box<dyn Error>> {
		let server = HttpServer::new(move || {
			App::new()
				.app_data(state.clone())
				.service(routes::get_home)
				.service(routes::get_collections)
				.service(routes::post_collection)
		})
			.bind(("127.0.0.1", 8080))?
			.run();

		Ok(server)
	}
} 

pub async fn run() -> Result<(), Box<dyn Error>> {
	let server = Server::new(web::Data::new(ServerState {
		collections: Mutex::new(vec![])
	})).await.unwrap();	
	println!("Server is running on http://localhost:8080");
	println!("Press Ctrl-C to close it");
	server.await?;
	Ok(())
}
