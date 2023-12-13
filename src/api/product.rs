use actix_web::{
  web::{get, post, resource, Json, Path},
  HttpResponse, Resource,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
struct Product {
  id: u32,
  name: String,
  price: u64,
}

#[derive(Deserialize)]
struct CreateProductDto {
  name: String,
  price: u64,
}

fn product_data() -> Vec<Product> {
  let data = vec![Product {
    id: 1,
    name: "Fashing Machine".to_string(),
    price: 50000,
  }];

  return data;
}

#[derive(Serialize)]
struct ResponseSuccess {
  status: u16,
  message: String,
  data: ResponseData,
}

#[derive(Serialize)]
#[serde(untagged)]
enum ResponseData {
  Single(Product),
  Multiple(Vec<Product>),
}

async fn get_all() -> HttpResponse {
  HttpResponse::Ok().json(ResponseSuccess {
    status: 200,
    message: String::from("Get all products"),
    data: ResponseData::Multiple(product_data()),
  })
}

async fn get_one(path: Path<u32>) -> HttpResponse {
  let id = path.into_inner();

  let product_data = product_data();
  let one_product: Option<&Product> = product_data.iter().find(|item: &&Product| item.id == id);

  HttpResponse::Ok().json(ResponseSuccess {
    status: 200,
    message: format!("Get product with id {}", id),
    data: ResponseData::Single(one_product.unwrap().clone()),
  })
}

async fn create(body: Json<CreateProductDto>) -> HttpResponse {
  HttpResponse::Ok().json(ResponseSuccess {
    status: 200,
    message: "Product created".to_string(),
    data: ResponseData::Single(Product {
      id: product_data().iter().last().unwrap().id + 1,
      name: body.name.to_string(),
      price: body.price,
    }),
  })
}

pub fn services() -> Vec<Resource> {
  let mut resources = Vec::new();

  resources.push(resource("/products").route(post().to(create)));
  resources.push(resource("/products").route(get().to(get_all)));
  resources.push(resource("/products/{id}").route(get().to(get_one)));

  resources
}
