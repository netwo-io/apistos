use actix_web::middleware::Logger;
use actix_web::web::{Json, Path};
use actix_web::{App, Error};

use actix_web::test::{TestRequest, call_service, init_service, try_read_body_json};
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::web::{ServiceConfig, delete, get, patch, post, put, resource, scope, tagged_resource, tagged_scope};
use apistos_gen::{ApiComponent, api_operation};
use apistos_models::OpenApi;
use apistos_models::info::Info;
use apistos_models::tag::Tag;
use schemars::JsonSchema;
use serde::Serialize;

#[actix_web::test]
async fn actix_routing() {
  fn my_routes(cfg: &mut ServiceConfig) {
    cfg.service(
      resource("/users/{user_id}")
        .route(delete().to(test))
        .wrap(Logger::default()),
    );
  }

  #[api_operation(tag = "pet")]
  pub(crate) async fn test(_params: Path<(u32, String)>) -> Result<Json<()>, Error> {
    Ok(Json(()))
  }

  let info = Info {
    title: "A well documented API".to_string(),
    description: Some("Really well document I mean it".to_string()),
    terms_of_service: Some("https://terms.com".to_string()),
    ..Default::default()
  };
  let tags = vec![
    Tag {
      name: "pet".to_owned(),
      ..Default::default()
    },
    Tag {
      name: "A super tag".to_owned(),
      ..Default::default()
    },
    Tag {
      name: "Another super tag".to_owned(),
      ..Default::default()
    },
  ];
  let spec = Spec {
    info: info.clone(),
    tags: tags.clone(),
    ..Default::default()
  };

  let app = App::new()
    .document(spec)
    .service(
      scope("test")
        .service(resource("/{plop_id}/{clap_name}").route(get().to(test)))
        .service(tagged_resource("/line/{plop_id}", vec!["A super tag".to_string()]).route(put().to(test)))
        .service(
          tagged_scope("test2", vec!["Another super tag".to_string()]).service(resource("/").route(post().to(test))),
        )
        .route("test3", patch().to(test))
        .route("test4/{test_id}", patch().to(test))
        .app_data("")
        .configure(my_routes)
        .service(scope("test5").route("", post().to(test)).route("", get().to(test)))
        .service(resource("test6").route(post().to(test)))
        .service(resource("test6").route(get().to(test))),
    )
    .build("/openapi.json");
  let app = init_service(app).await;

  let req = TestRequest::get().uri("/openapi.json").to_request();
  let resp = call_service(&app, req).await;
  assert!(resp.status().is_success());

  let body: OpenApi = try_read_body_json(resp).await.expect("Unable to read body");
  let mut paths: Vec<&String> = body.paths.paths.keys().collect();
  paths.sort();

  let expected_paths = vec![
    "/test/line/{plop_id}",
    "/test/test2/",
    "/test/test3",
    "/test/test4/{test_id}",
    "/test/test5",
    "/test/test6",
    "/test/users/{user_id}",
    "/test/{plop_id}/{clap_name}",
  ];

  assert_eq!(paths, expected_paths);

  assert_eq!(
    body.paths.paths.values().flat_map(|v| v.operations.values()).count(),
    10
  );
}

#[actix_web::test]
async fn actix_routing_multiple_root_definition_holder() {
  #[derive(ApiComponent, JsonSchema, Serialize)]
  pub(crate) struct TestResponse {
    pub(crate) value: String,
  }

  #[derive(ApiComponent, JsonSchema, Serialize)]
  pub(crate) struct TestResponse2 {
    pub(crate) value: String,
  }

  #[api_operation(tag = "pet")]
  pub(crate) async fn test(_params: Path<(u32, String)>) -> Result<Json<TestResponse>, Error> {
    Ok(Json(TestResponse {
      value: "plop".to_string(),
    }))
  }

  #[api_operation(tag = "pet")]
  pub(crate) async fn test2(_params: Path<String>) -> Result<Json<TestResponse2>, Error> {
    Ok(Json(TestResponse2 {
      value: "plop".to_string(),
    }))
  }

  let info = Info {
    title: "A well documented API".to_string(),
    description: Some("Really well document I mean it".to_string()),
    terms_of_service: Some("https://terms.com".to_string()),
    ..Default::default()
  };
  let tags = vec![
    Tag {
      name: "pet".to_owned(),
      ..Default::default()
    },
    Tag {
      name: "A super tag".to_owned(),
      ..Default::default()
    },
    Tag {
      name: "Another super tag".to_owned(),
      ..Default::default()
    },
  ];
  let spec = Spec {
    info: info.clone(),
    tags: tags.clone(),
    ..Default::default()
  };

  let app = App::new()
    .document(spec)
    .service(scope("test").service(resource("/{plop_id}/{clap_name}").route(get().to(test).wrap(Logger::default()))))
    .service(scope("test2").service(resource("/{clap_name}").route(get().to(test2))))
    .build("/openapi.json");
  let app = init_service(app).await;

  let req = TestRequest::get().uri("/openapi.json").to_request();
  let resp = call_service(&app, req).await;
  assert!(resp.status().is_success());

  let body: OpenApi = try_read_body_json(resp).await.expect("Unable to read body");
  let components = body.components.expect("Unable to get components");

  assert_eq!(components.schemas.len(), 2);
}

// Imports bellow aim at making clippy happy. Those dependencies are necessary for integration-test.
use actix_service as _;
use actix_web_lab as _;
use apistos_core as _;
use apistos_plugins as _;
use apistos_rapidoc as _;
use apistos_redoc as _;
use apistos_scalar as _;
use apistos_swagger_ui as _;
use assert_json_diff as _;
use futures_util as _;
use garde_actix_web as _;
use indexmap as _;
use log as _;
use md5 as _;
use once_cell as _;
use regex as _;
use serde_json as _;
