# Apistos &emsp; [![Documentation]][docs.rs] [![Latest Version]][crates.io] [![Build Status]][build]

[docs.rs]: https://docs.rs/apistos/

[crates.io]: https://crates.io/crates/apistos

[build]: https://github.com/netwo-io/apistos/actions/workflows/build.yaml?branch=main

[Documentation]: https://img.shields.io/docsrs/apistos

[Latest Version]: https://img.shields.io/crates/v/apistos.svg

[Build Status]: https://github.com/netwo-io/apistos/actions/workflows/build.yaml/badge.svg?branch=main

[OASv3.md]: https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md

An actix-web wrapper similar to [paperclip](https://github.com/paperclip-rs/paperclip) to generate [OAS 3.0][OASv3.md]
documentation.

- [Installation](#installation)
- [Usage example](#usage-example)
- [Feature flags](#feature-flags)
- [About us](#about-us)

### Installation

```toml
[dependencies]
#schemars = "0.8"
# sadly we currently rely on a fork to fix multiple flatten for enums, related PR can be found here: https://github.com/GREsau/schemars/pull/264
schemars = { package = "apistos-schemars", version = "0.8" }
apistos = "0.2"
```

### Usage example

Wrap your regular actix-web app using **apistos** types.

Most of these types are drop-in types for actix-web one's.

```rust
use actix_web::{App, HttpServer};
use actix_web::web::Json;
use apistos::actix::CreatedJson;
use apistos::api_operation;
use apistos::ApiComponent;
use apistos::ApiErrorComponent;
use apistos::spec::Spec;
use apistos::web::{post, resource, scope};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub struct Test {
  pub test: String
}

#[derive(Serialize, Deserialize, Clone, ApiErrorComponent)]
#[openapi_error(
  status(code = 403),
  status(code = 404),
  status(code = 405, description = "Invalid input"),
  status(code = 409)
)]
pub enum ErrorResponse {
  MethodNotAllowed(String),
  NotFound(String),
  Conflict(String),
  Unauthorized(String),
}

#[api_operation(
  tag = "pet",
  summary = "Add a new pet to the store",
  description = r###"Add a new pet to the store
    Plop"###,
  error_code = 405
)]
pub(crate) async fn test(
  body: Json<Test>,
) -> Result<CreatedJson<Test>, ErrorResponse> {
  Ok(CreatedJson(body.0))
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
  HttpServer::new(move || {
    let spec = Spec {
      info: Info {
        title: "An API".to_string(),
        version: "1.0.0".to_string(),
        ..Default::default()
      },
      ..Default::default()
    };

    App::new()
      .document(spec)
      .wrap(Logger::default())
      .service(scope("/test")
        .service(
          resource("")
            .route(post().to(test))
        )
      )
      .build("/openapi.json")
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
```

For a complete example, see [the sample petstore](https://github.com/netwo-io/apistos/tree/main/examples/petstore).

### Feature flags

| name           | description                                                              | extra dependencies                                            |
|----------------|--------------------------------------------------------------------------|---------------------------------------------------------------|
| `chrono`       | Enables documenting types from `chrono`                                  | [`chrono`](https://crates.io/crates/chrono)                   |
| `multipart`    | Enables documenting types from `actix-multipart`                         | [`actix-multipart`](https://crates.io/crates/actix-multipart) |
| `rust_decimal` | Enables documenting types from `rust_decimal`                            | [`rust_decimal`](https://crates.io/crates/rust-decimal) =     |
| `uuid`         | Enables documenting types from `uuid`                                    | [`uuid`](https://crates.io/crates/uuid)                       |
| `url`          | Enables documenting types from `url`                                     | [`url`](https://crates.io/crates/url)                         |
| `extras`       | Enables `chrono`, `multipart`, `rust_decimal`, `uuid` and `url` features | All from previous features                                    |

### About us

apistos is provided by [Netwo](https://www.netwo.io).

We use this crate for our internal needs and therefore are committed to its maintenance, however we cannot provide any
additional guaranty. Use it at your own risks.

While we won't invest in any feature we don't need, we are open to accept any pull request you might propose.

We are a France based full-remote company operating in the telecom industry. If you are interested in learning more,
feel free to visit [our career page](https://www.netwo.io/carriere).
