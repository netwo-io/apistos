use crate::api::{add_todo, get_todo};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::info::Info;
use apistos::server::Server;
use apistos::spec::Spec;
use apistos::web::{get, post, resource, scope};
use apistos::{OpenApiVersion, RapidocConfig, RedocConfig, ScalarConfig, SwaggerUIConfig};
use std::error::Error;
use std::net::Ipv4Addr;

mod api;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
  HttpServer::new(move || {
    let spec = Spec {
      openapi: OpenApiVersion::OAS3_1,
      info: Info {
        title: "A well documented API".to_string(),
        description: Some(
          "This is an API documented using Apistos,\na wonderful new tool to document your actix API !".to_string(),
        ),
        ..Default::default()
      },
      servers: vec![Server {
        url: "/api/v3".to_string(),
        ..Default::default()
      }],
      ..Default::default()
    };

    App::new()
      .document(spec)
      .wrap(Logger::default())
      .service(
        scope("/test").service(
          scope("/todo")
            .service(resource("/{todo_id}").route(get().to(get_todo)))
            .service(resource("").route(post().to(add_todo))),
        ),
      )
      .build_with(
        "/openapi.json",
        BuildConfig::default()
          .with(RapidocConfig::new(&"/rapidoc"))
          .with(RedocConfig::new(&"/redoc"))
          .with(ScalarConfig::new(&"/scalar"))
          .with(SwaggerUIConfig::new(&"/swagger")),
      )
  })
  .bind((Ipv4Addr::UNSPECIFIED, 8080))?
  .run()
  .await
}
