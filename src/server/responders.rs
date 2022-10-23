use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use actix_web::body::BoxBody;
use super::super::database::model::{Item, Collection};

impl Responder for Item {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		let res_body = serde_json::to_string(&self).unwrap();

		HttpResponse::Ok()
			.content_type(ContentType::json())
			.body(res_body)
	}
}

impl Responder for Collection {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		let res_body = serde_json::to_string(&self).unwrap();

		HttpResponse::Ok()
			.content_type(ContentType::json())
			.body(res_body)
	}
}
