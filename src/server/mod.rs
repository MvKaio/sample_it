use actix_web::{web, dev, HttpServer, App};
use std::error::Error;
use std::sync::Mutex;

use crate::database;
use rusqlite::Connection;

use super::database::model::Collection;

mod responders;
mod routes;

pub struct ServerState {
	collections: Mutex<Vec<Collection>>,
    connection: Mutex<Connection>
}

pub struct Server;

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
		collections: Mutex::new(vec![]),
        connection: Mutex::new(database::connect().unwrap())
	})).await.unwrap();	
	println!("Server is running on http://localhost:8080");
	println!("Press Ctrl-C to close it");
	server.await?;
	Ok(())
}
