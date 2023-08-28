use crate::api::routes::routes;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use netwopenapi::app::OpenApiWrapper;
use netwopenapi::spec::Spec;
use netwopenapi::web::scope;
use std::error::Error;
use std::net::Ipv4Addr;
use utoipa::openapi::external_docs::ExternalDocsBuilder;
use utoipa::openapi::tag::TagBuilder;
use utoipa::openapi::{ContactBuilder, Info, InfoBuilder, LicenseBuilder, ServerBuilder};

mod api;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
  env_logger::init();

  HttpServer::new(move || {
    let spec = Spec {
      default_tags: vec!["api".to_owned()],
      tags: vec![
        TagBuilder::new()
          .name("api")
          .description(Some("Everything about petstore"))
          .build(),
        TagBuilder::new()
          .name("pet")
          .description(Some("Everything about your Pets"))
          .build(),
        TagBuilder::new()
          .name("store")
          .description(Some("Access to Petstore orders"))
          .build(),
        TagBuilder::new()
          .name("user")
          .description(Some("Operations about user"))
          .build(),
      ],
      info: InfoBuilder::new()
        .title("Swagger Petstore - OpenAPI 3.0")
        .description(Some("This is a sample Pet Store Server based on the OpenAPI 3.0 specification.  You can find out more about\nSwagger at [http://swagger.io](http://swagger.io). In the third iteration of the pet store, we've switched to the design first approach!\nYou can now help us improve the API whether it's by making changes to the definition itself or to the code.\nThat way, with time, we can improve the API in general, and expose some of the new features in OAS3.\n\nSome useful links:\n- [The Pet Store repository](https://github.com/swagger-api/swagger-petstore)\n- [The source API definition for the Pet Store](https://github.com/swagger-api/swagger-petstore/blob/master/src/main/resources/openapi.yaml)"))
        .terms_of_service(Some("http://swagger.io/terms/"))
        .contact(Some(ContactBuilder::new().email(Some("apiteam@swagger.io")).build()))
        .license(Some(LicenseBuilder::new().name("Apache 2.0").url(Some("http://www.apache.org/licenses/LICENSE-2.0.html")).build()))
        .version("1.0.17")
        .build(),
      external_docs: Some(ExternalDocsBuilder::new().description(Some("Find out more about Swagger")).url("http://swagger.io").build()),
      servers: vec![ServerBuilder::new().url("/api/v3").build()],
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
