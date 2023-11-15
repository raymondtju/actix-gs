pub mod api;

use actix_web::{
  web::{self, route},
  App, HttpServer,
};
use api::error_handler::not_found;

// This struct represents state
pub struct AppState {
  app_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .app_data(web::Data::new(AppState {
        app_name: String::from("Actix Web"),
      }))
      .service(api::base::services())
      .service(api::product::services())
      .default_service(route().to(not_found))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
