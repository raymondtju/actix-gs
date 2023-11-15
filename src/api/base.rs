use actix_web::{
  web::{self, get, resource},
  HttpResponse, Resource, Responder,
};
use serde_json::json;

use crate::AppState;

async fn index(data: web::Data<AppState>) -> impl Responder {
  let app_name = &data.app_name;
  HttpResponse::Ok().json(json!({"name": format!("Hello {app_name}"),}))
}

pub fn services() -> Resource {
  resource("/").route(get().to(index))
}
