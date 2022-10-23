use actix_web::{web, get, post, put, delete, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use crate::database;

use super::ServerState;
use super::super::database::model::{Item, Collection};

#[get("/")]
pub async fn get_home() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello :)</h1>")
}

#[get("/collections")]
async fn get_collections(data: web::Data<ServerState>) -> impl Responder {
    let connection = data.connection.lock().unwrap();
    let collections = database::functions::get_collections(&connection).unwrap();
	let response = serde_json::to_string(&collections).unwrap();

	HttpResponse::Ok()
		.content_type(ContentType::json())
		.body(response)
}

#[post("/collections")]
async fn post_collection(req: web::Json<Collection>, data: web::Data<ServerState>) -> impl Responder {
	println!("[INFO][POST /collections] items are being ignored for now");

	//let mut _collections = data.collections.lock().unwrap();
	//let response = serde_json::to_string(&new_collection).unwrap();

	HttpResponse::Created()
		.content_type(ContentType::json())
		.body("")
}
