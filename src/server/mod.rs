use actix_web::{web, dev, HttpServer, App};
use std::error::Error;
use std::sync::Mutex;

use crate::database;
use rusqlite::Connection;

mod responders;
mod routes;

pub struct ServerState {
    connection: Mutex<Connection>
}

pub struct Server;

impl Server {
	pub async fn start(state: web::Data<ServerState>) -> Result<dev::Server, Box<dyn Error>> {
		let server = HttpServer::new(move || {
			let cors = Cors::default().allow_any_origin().send_wildcard();
			App::new()
				.wrap(cors)
				.app_data(state.clone())
				.service(routes::get_home)
				.service(routes::get_collections)
				.service(routes::post_collection)
		})
			.bind(("127.0.0.1", 3000))?
			.run();

		Ok(server)
	}
} 

pub async fn run() -> Result<(), Box<dyn Error>> {
	let server = Server::start(web::Data::new(ServerState {
        connection: Mutex::new(database::connect().unwrap())
	})).await.unwrap();	
	println!("Server is running on http://localhost:8080");
	println!("Press Ctrl-C to close it");
	server.await?;
	Ok(())
}
