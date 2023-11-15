use actix_web::{
  web::{get, resource, Query},
  HttpResponse, Resource, Responder,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct ProductQuery {
  product_id: Option<u32>,
}

async fn get_one(query: Query<ProductQuery>) -> impl Responder {
  HttpResponse::Ok().json(json!({
    "query": query.product_id
  }))
}

pub fn services() -> Resource {
  resource("/product").route(get().to(get_one))
}
