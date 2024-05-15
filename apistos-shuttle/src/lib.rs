use std::net::SocketAddr;

use actix_web::App;

use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;

#[derive(Clone)]
pub struct ApistosActixWebService<T> {
  pub spec: Spec,
  pub service_config: T,
  pub openapi_path: String,
}

#[shuttle_runtime::async_trait]
impl<T> shuttle_runtime::Service for ApistosActixWebService<T>
where
  T: FnOnce(&mut apistos::web::ServiceConfig) + Send + Clone + 'static,
{
  async fn bind(mut self, addr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
    // Start a worker for each cpu, but no more than 4.
    let worker_count = num_cpus::get().min(4);

    let server = actix_web::HttpServer::new(move || {
      let spec = self.spec.clone();
      App::new()
        .document(spec)
        .configure(self.service_config.clone())
        .build(&self.openapi_path)
    })
    .workers(worker_count)
    .bind(addr)?
    .run();

    server.await.map_err(shuttle_runtime::CustomError::new)?;

    Ok(())
  }
}

#[doc = include_str!("../README.md")]
pub type ShuttleApistosActixWeb<T> = Result<ApistosActixWebService<T>, shuttle_runtime::Error>;
