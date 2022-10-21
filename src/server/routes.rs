use actix_web::{web, get, post, put, delete, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use chrono::DateTime;

use crate::database;

use super::ServerState;
use super::super::database::model::{Item, Label, Collection};

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
	let new_collection = Collection::new(
		req.id,
		req.created_at,
		req.updated_at,
		String::from(&req.title),
		String::from(&req.description),
		vec![]);

	println!("[INFO][POST /collections] items are being ignored for now");

	let mut collections = data.collections.lock().unwrap();
	let response = serde_json::to_string(&new_collection).unwrap();

	collections.push(new_collection);
	HttpResponse::Created()
		.content_type(ContentType::json())
		.body(response)
}

// #[get("/collections")]
// #[get("/collection/<id>")]
// #[get("/collection/<id>/items")]
// #[get("/collection/<id>/labels")]
// #[get("/collection/<id>/item/<id>")]
// #[get("/collection/<id>/label/<id>")]
