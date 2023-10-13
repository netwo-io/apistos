use crate::api::{add_todo, get_todo};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use apistos::app::OpenApiWrapper;
use apistos::info::{Contact, Info, License};
use apistos::paths::ExternalDocumentation;
use apistos::server::Server;
use apistos::spec::Spec;
use apistos::tag::Tag;
use apistos::web::{get, post, resource, scope};
use std::error::Error;
use std::net::Ipv4Addr;

mod api;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
  HttpServer::new(move || {
    let spec = Spec {
      default_tags: vec![],
      tags: vec![Tag {
        name: "todo".to_string(),
        description: Some("A simple description for this tag".to_string()),
        ..Default::default()
      }],
      info: Info {
        title: "A well documented API".to_string(),
        description: Some(
          "This is an API documented using Apistos,\na wonderful new tool to document your actix API !".to_string(),
        ),
        contact: Some(Contact {
          email: Some("apiteam@netwo.io".to_string()),
          ..Default::default()
        }),
        license: Some(License {
          name: "Apache 2.0".to_string(),
          url: Some("http://www.apache.org/licenses/LICENSE-2.0.html".to_string()),
          ..Default::default()
        }),
        version: "1.0.17".to_string(),
        ..Default::default()
      },
      external_docs: Some(ExternalDocumentation {
        description: Some("Find out more about Apistos".to_string()),
        url: "https://github.com/netwo-io/apistos".to_string(),
        ..Default::default()
      }),
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
      .build("/openapi.json")
  })
  .bind((Ipv4Addr::UNSPECIFIED, 8080))?
  .run()
  .await
}
