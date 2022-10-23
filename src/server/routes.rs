use actix_web::{web, get, post, put, delete, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use crate::database;
use crate::database::model::Collection;

use super::ServerState;

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
    let new_collection = req.into_inner();

    let connection = data.connection.lock().unwrap();
    let new_collection = database::functions::push_collection(&new_collection, &connection).unwrap();
	let response = serde_json::to_string(&new_collection).unwrap();

	HttpResponse::Created()
		.content_type(ContentType::json())
		.body(response)
}
