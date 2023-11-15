use actix_web::{http::StatusCode, HttpResponse, Responder};
use serde_json::json;

pub async fn not_found() -> impl Responder {
  HttpResponse::Ok()
    .status(StatusCode::NOT_FOUND)
    .json(json!({
        "message": "not found"
    }))
}
