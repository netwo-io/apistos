use crate::api::routes::routes;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use apistos::app::OpenApiWrapper;
use apistos::info::{Contact, Info, License};
use apistos::paths::ExternalDocumentation;
use apistos::server::Server;
use apistos::spec::Spec;
use apistos::tag::Tag;
use apistos::web::scope;
use apistos::OpenApiVersion;
use std::error::Error;
use std::net::Ipv4Addr;

mod api;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
  env_logger::init();

  HttpServer::new(move || {
    let spec = Spec {
      openapi: OpenApiVersion::OAS3_1,
      default_tags: vec!["api".to_owned()],
      tags: vec![
        Tag {
          name: "api".to_string(),
          description: Some("Everything about petstore".to_string()),
          ..Default::default()
        },
        Tag {
          name: "pet".to_string(),
          description: Some("Everything about your Pets".to_string()),
          ..Default::default()
        },
        Tag {
          name: "store".to_string(),
          description: Some("Access to Petstore orders".to_string()),
          ..Default::default()
        },
        Tag {
          name: "user".to_string(),
          description: Some("Operations about user".to_string()),
          ..Default::default()
        },
      ],
      info: Info {
        title: "Swagger Petstore - OpenAPI 3.0".to_string(),
        description: Some("This is a sample Pet Store Server based on the OpenAPI 3.0 specification.  You can find out more about\nSwagger at [http://swagger.io](http://swagger.io). In the third iteration of the pet store, we've switched to the design first approach!\nYou can now help us improve the API whether it's by making changes to the definition itself or to the code.\nThat way, with time, we can improve the API in general, and expose some of the new features in OAS3.\n\nSome useful links:\n- [The Pet Store repository](https://github.com/swagger-api/swagger-petstore)\n- [The source API definition for the Pet Store](https://github.com/swagger-api/swagger-petstore/blob/master/src/main/resources/openapi.yaml)".to_string()),
        terms_of_service: Some("http://swagger.io/terms/".to_string()),
        contact: Some(Contact {
          email: Some("apiteam@swagger.io".to_string()),
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
        description: Some("Find out more about Swagger".to_string()),
        url: "http://swagger.io".to_string(),
        ..Default::default()
      }),
      servers: vec![Server { url: "/api/v3".to_string(), ..Default::default() }],
      ..Default::default()
    };

    App::new()
      .document(spec)
      .wrap(Logger::default())
      .service(scope("/test").service(routes()))
      .build("/openapi.json")
  })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
      .workers(1)
    .run()
    .await
}
