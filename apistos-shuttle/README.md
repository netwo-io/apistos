# Apistos Shuttle &emsp; [![Documentation]][docs.rs] [![Latest Version]][crates.io] [![Build Status]][build] [![Deps Status]][deps.rs]

[docs.rs]: https://docs.rs/apistos-shuttle/

[crates.io]: https://crates.io/crates/apistos-shuttle

[build]: https://github.com/netwo-io/apistos/actions/workflows/build.yaml?branch=main

[Documentation]: https://img.shields.io/docsrs/apistos-shuttle

[Latest Version]: https://img.shields.io/crates/v/apistos-shuttle.svg

[Build Status]: https://github.com/netwo-io/apistos/actions/workflows/build.yaml/badge.svg?branch=main

[deps.rs]: https://deps.rs/crate/apistos-shuttle

[Deps Status]: https://deps.rs/crate/apistos-shuttle/latest/status.svg

This crate allows you to run an actix-web server documented with Apistos on [Shuttle](https://www.shuttle.rs/).

### Installation

```toml
[dependencies]
#schemars = "0.8"
# sadly we currently rely on a fork to fix multiple flatten for enums, related PR can be found here: https://github.com/GREsau/schemars/pull/264
schemars = { package = "apistos-schemars", version = "0.8" }
apistos = { version = "0.5" }
apistos-shuttle = { version = "0.5" }
```

### Example

```rust
use actix_web::web::Json;
use actix_web::Error;

use apistos::api_operation;
use apistos::info::Info;
use apistos::spec::Spec;
use apistos::web::{get, resource, ServiceConfig};
use apistos_shuttle::{ApistosActixWebService, ShuttleApistosActixWeb};

#[api_operation(summary = "Say 'Hello world!'")]
pub(crate) async fn hello_world() -> Result<Json<String>, Error> {
  Ok(Json("Hello world!".to_string()))
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleApistosActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
  let spec = Spec {
    info: Info {
      title: "A well documented API running on Shuttle".to_string(),
      description: Some(
        "This is an API documented using Apistos,\na wonderful new tool to document your actix API !".to_string(),
      ),
      ..Default::default()
    },
    ..Default::default()
  };

  let service_config = move |cfg: &mut ServiceConfig| {
    cfg.service(resource("/").route(get().to(hello_world)));
  };

  Ok(ApistosActixWebService {
    spec,
    service_config,
    openapi_path: "/openapi".to_string(),
  })
}
```

> [!IMPORTANT]  
> Default features are disabled for `shuttle-runtime` to avoid pulling [colored](https://github.com/colored-rs/colored)
> which as a license considered as
> copyleft by cargo-deny.
> To implement your own tracing, please visit <https://docs.shuttle.rs/configuration/logs>

### About us

apistos is provided by [Netwo](https://www.netwo.io).

We use this crate for our internal needs and therefore are committed to its maintenance, however we cannot provide any
additional guaranty. Use it at your own risks.

While we won't invest in any feature we don't need, we are open to accept any pull request you might propose.

We are a France based full-remote company operating in the telecom industry. If you are interested in learning more,
feel free to visit [our career page](https://www.netwo.io/carriere).
