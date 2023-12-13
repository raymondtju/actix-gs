pub mod api;

use actix_web::{
  error,
  web::{self, route},
  App, Error, HttpRequest, HttpResponse, HttpServer,
};
use api::error_handler::not_found;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// This struct represents state
pub struct AppState {
  app_name: String,
}

#[derive(Serialize, Debug)]
struct BadRequestError {
  status: i32,
  error: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiResult<T = ()> {
  pub status: i32,
  pub message: Option<Cow<'static, str>>,
  pub data: Option<T>,
  pub error: Option<Cow<'static, str>>,
}

// return 200 all
pub fn json_error_handler<E: std::fmt::Display + std::fmt::Debug + 'static>(
  err: E,
  req: &HttpRequest,
) -> error::Error {
  let detail = err.to_string();
  let api = ApiResult::new().with_data(()).code(400).with_err(detail);
  let response = api.log_to_resp(req);

  error::InternalError::from_response(err, response).into()
}

impl<T: Serialize> ApiResult<T> {
  pub fn new() -> Self {
    Self {
      status: 200,
      message: None,
      data: None,
      error: None,
    }
  }
  pub fn code(mut self, status: i32) -> Self {
    self.status = status;
    self
  }
  pub fn with_msg<S: Into<Cow<'static, str>>>(mut self, message: S) -> Self {
    self.message = Some(message.into());
    self
  }
  pub fn with_err<S: Into<Cow<'static, str>>>(mut self, error: S) -> Self {
    self.error = Some(error.into());
    self
  }
  pub fn msg_as_str(&self) -> &str {
    self
      .message
      .as_ref()
      .map(|s| s.as_ref())
      .unwrap_or_default()
  }
  pub fn with_data(mut self, data: T) -> Self {
    self.data = Some(data);
    self
  }
  pub fn log_to_resp(&self, req: &HttpRequest) -> HttpResponse {
    self.log(req);
    self.to_resp()
  }
  pub fn log(&self, req: &HttpRequest) {
    info!(
      "{} \"{} {} {:?}\" {}",
      req.peer_addr().unwrap(),
      req.method(),
      req.uri(),
      req.version(),
      self.status,
    );
  }
  pub fn to_resp(&self) -> HttpResponse {
    let resp = match serde_json::to_string(self) {
      Ok(json) => HttpResponse::Ok()
        .content_type("application/json")
        .body(json),
      Err(e) => Error::from(e).into(),
    };

    resp
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .app_data(web::Data::new(AppState {
        app_name: String::from("Actix Web"),
      }))
      .app_data(web::JsonConfig::default().error_handler(json_error_handler))
      .service(api::base::services())
      .service(api::product::services())
      .default_service(route().to(not_found))
  })
  .bind(("127.0.0.1", 8080))
  .unwrap()
  .run()
  .await
}
