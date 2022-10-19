use actix_web::{dev, get, HttpServer, App, HttpResponse, Responder};
use std::error::Error;

#[derive(Clone)]
pub struct ServerState {
	collections: Vec<String>
}

pub struct Server;

#[get("/")]
async fn get_home() -> impl Responder {
	HttpResponse::Ok().body("
		<h4>If you are seeing this message, the server is running. Sample as dat :)</h4>
	")
}

impl Server {
	pub async fn new(state: ServerState) -> Result<dev::Server, Box<dyn Error>> {
		let server = HttpServer::new(move || {
			App::new()
				.app_data(state.clone())
				.service(get_home)
		})
			.bind(("127.0.0.1", 8080))?
			.run();

		Ok(server)
	}
} 

pub async fn run() -> Result<(), Box<dyn Error>> {
	let server = Server::new(ServerState {
		collections: vec![String::from("Collection 1"), String::from("Not collection 1")]
	}).await.unwrap();	
	println!("Server is running on http://localhost:8080");
	println!("Press Ctrl-C to close it");
	server.await?;
	Ok(())
}
