use crate::api::routes::routes;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use netwopenapi::app::OpenApiWrapper;
use netwopenapi::spec::Spec;
use netwopenapi::web::scope;
use std::error::Error;
use std::net::Ipv4Addr;

mod api;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
  env_logger::init();

  HttpServer::new(move || {
    let spec = Spec {
      default_tags: vec!["api".to_owned()],
      ..Default::default()
    };

    App::new()
      .document(spec)
      .wrap(Logger::default())
      .service(scope("/test").service(routes()))
      .build("/openapi.json")
  })
  .workers(1)
  .bind((Ipv4Addr::UNSPECIFIED, 8080))?
  .run()
  .await
}
