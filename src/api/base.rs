use crate::AppState;
use actix_web::{
  web::{self, get, resource},
  HttpResponse, Resource, Responder,
};
use chrono;
use serde_json::json;

async fn index(data: web::Data<AppState>) -> impl Responder {
  let app_name = &data.app_name;
  HttpResponse::Ok().json(json!({"name": format!("Hello {app_name}"),}))
}

async fn server_time() -> impl Responder {
  HttpResponse::Ok().body(chrono::Utc::now().to_string())
}

pub fn services() -> Resource {
  resource("/").route(get().to(index));
  resource("/time").route(get().to(server_time))
}
