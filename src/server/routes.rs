use actix_web::{web, get, post, put, delete, HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use crate::database;
use crate::database::model::{Collection, Sample};

use super::ServerState;
use crate::solver;

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
    let new_collection = database::functions::push_collection(&new_collection, &connection, false).unwrap();
	let response = serde_json::to_string(&new_collection).unwrap();

	HttpResponse::Created()
		.content_type(ContentType::json())
		.body(response)
}

#[get("/collections/{id}")]
async fn get_collection(req: HttpRequest, data: web::Data<ServerState>) -> impl Responder {
    let id: u32 = req.match_info().get("id").unwrap().parse().unwrap();
    let connection = data.connection.lock().unwrap();

    let collection = database::functions::get_collection(id, &connection).unwrap();
    let response = serde_json::to_string(&collection).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

#[delete("/collections/{id}")]
async fn delete_collection(req: HttpRequest, data: web::Data<ServerState>) -> impl Responder {
    let id: u32 = req.match_info().get("id").unwrap().parse().unwrap();
    let connection = data.connection.lock().unwrap();

    database::functions::delete_collection(id, &connection).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("Collection Deleted Successfully")
}

#[put("/collections/{id:\\d+}")]
async fn update_collection(req: web::Path<u32>, collection: web::Json<Collection>, data: web::Data<ServerState>) -> impl Responder {
    let id: u32 = req.into_inner();
    let connection = data.connection.lock().unwrap();

    let collection = database::functions::update_collection(id, &collection, &connection).unwrap();
    let response = serde_json::to_string(&collection).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

#[post("/samples")]
async fn post_sample(req: web::Json<Sample>, data: web::Data<ServerState>) -> impl Responder {
    let sample = req.into_inner();
    let connection = data.connection.lock().unwrap();

    let solution = solver::solve(sample, &connection).unwrap();
	let response = serde_json::to_string(&solution).unwrap();

	HttpResponse::Created()
		.content_type(ContentType::json())
		.body(response)
}
