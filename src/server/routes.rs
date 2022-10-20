use actix_web::{web, get, post, put, delete, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use super::ServerState;

#[get("/")]
pub async fn get_home() -> impl Responder {
    HttpResponse::Ok().body("<h1>HTML</h1>")
}

#[get("/collections")]
async fn get_collections(data: web::Data<ServerState>) -> impl Responder {
	let collections = data.collections.lock().unwrap();
	let response = serde_json::to_string(&(*collections)).unwrap();
	HttpResponse::Ok()
		.content_type(ContentType::json())
		.body(response)
}

// #[get("/collections")]
// #[get("/collection/<id>")]
// #[get("/collection/<id>/items")]
// #[get("/collection/<id>/labels")]
// #[get("/collection/<id>/item/<id>")]
// #[get("/collection/<id>/label/<id>")]
