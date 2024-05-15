use apistos::info::Info;
use apistos::server::Server;
use apistos::spec::Spec;
use apistos::web::{get, post, resource, scope, ServiceConfig};
use apistos_shuttle::{ApistosActixWebService, ShuttleApistosActixWeb};

use crate::api::{add_todo, get_todo};

mod api;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleApistosActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
  let spec = Spec {
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

  let service_config = move |cfg: &mut ServiceConfig| {
    cfg.service(
      scope("/test").service(
        scope("/todo")
          .service(resource("/{todo_id}").route(get().to(get_todo)))
          .service(resource("").route(post().to(add_todo))),
      ),
    );
  };

  Ok(ApistosActixWebService {
    spec,
    service_config,
    openapi_path: "/openapi".to_string(),
  })
}
