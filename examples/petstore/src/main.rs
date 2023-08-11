use crate::api::routes::routes;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use netwopenapi::app::OpenApiWrapper;
use std::error::Error;
use std::net::Ipv4Addr;
use netwopenapi::spec::Spec;
use netwopenapi::web::scope;

mod api;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
  env_logger::init();

  HttpServer::new(move || {
    App::new()
      .document(Spec::default())
      .wrap(Logger::default())
      .service(scope("/test").service(routes()))
      .build("/openapi.json")
  })
  .workers(1)
  .bind((Ipv4Addr::UNSPECIFIED, 8080))?
  .run()
  .await
}
