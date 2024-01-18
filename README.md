# Apistos &emsp; [![Documentation]][docs.rs] [![Latest Version]][crates.io] [![Build Status]][build]


[docs.rs]: https://docs.rs/apistos/
[crates.io]: https://crates.io/crates/apistos
[build]: https://github.com/netwo-io/apistos/actions/workflows/build.yaml?branch=main
[Documentation]: https://img.shields.io/docsrs/apistos
[Latest Version]: https://img.shields.io/crates/v/apistos.svg
[Build Status]: https://github.com/netwo-io/apistos/actions/workflows/build.yaml/badge.svg?branch=main

[OASv3.md]: https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md

An OpenAPI documentation tool exposing [OAS 3.0][OASv3.md] models as well as an actix-web wrapper similar to [paperclip](https://github.com/paperclip-rs/paperclip).

**Apistos** is composed of three crates:
- [`apistos`](./apistos): [actix-web](https://github.com/actix/actix-web) wrapper to generate an OpenAPI v3.0.3 documentation file
- [`apistos-core`](./apistos-core): A set of traits and common models around [OpenAPI v3.0.3][OASv3.md]
- [`apistos-gen`](./apistos-gen): macro utilities to generate [OpenAPI v3.0.3][OASv3.md] documentation from Rust models
- [`apistos-models`](./apistos-models): [OpenAPI v3.0.3][OASv3.md] models with [`Schema`](https://docs.rs/schemars/latest/schemars/schema/enum.Schema.html) based on [schemars](https://github.com/GREsau/schemars) definition 

Check out our [example project](examples/petstore).

### What does Apistos means

Apistos (pronounced **_/a.p.i.stos/_**) is a word play between Héphaïstos (Ἥφαιστος, grec god of blacksmiths, carpenters, craftsmen, metallurgy ... which can also be considered by some as the god of technology) and API (pronounced **_/a.p.i/_** in French).

## Apistos

- [Installation](#installation)
- [Usage example](#usage-example)
- [Feature flags](#feature-flags)
- [Alternatives](#alternatives)
- [About us](#about-us)

### Installation

```toml
[dependencies]
#schemars = "0.8"
# sadly we currently rely on a fork to fix multiple flatten for enums, related PR can be found here: https://github.com/GREsau/schemars/pull/250
schemars = { package = "apistos-schemars", version = "0.8" }
apistos = "0.1"
```

### Usage example

Wrap your regular actix-web app using apistos types. 

Most of these types are drop-in types for actix-web one's.

```rust
use std::fmt::Display;
use actix_web::{App, HttpServer, ResponseError};
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::web::Json;
use apistos::actix::CreatedJson;
use apistos::api_operation;
use apistos::ApiComponent;
use apistos::ApiErrorComponent;
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::web::{post, resource, scope};
use apistos_models::info::Info;
use core::fmt::Formatter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub struct Test {
  pub test: String
}

#[derive(Serialize, Deserialize, Debug, Clone, ApiErrorComponent)]
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

impl Display for ErrorResponse {
  fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl ResponseError for ErrorResponse {
  fn status_code(&self) -> StatusCode {
    todo!()
  }
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

| name              | description                                                              | extra dependencies                                            |
|-------------------|--------------------------------------------------------------------------|---------------------------------------------------------------|
| `query` (default) | Enables documenting `actix_web::web::Query`                              |                                                               |
| `actix` (default) | Enables documenting types from `actix`                                   |                                                               |
| `garde`           | Enables input validation through `garde`                                 | [`garde`](https://crates.io/crates/garde)                     |
| `qs_query`        | Enables documenting types from `serde_qs`                                | [`serde_qs`](https://crates.io/crates/serde-qs)               |
| `chrono`          | Enables documenting types from `chrono`                                  | [`chrono`](https://crates.io/crates/chrono)                   |
| `multipart`       | Enables documenting types from `actix-multipart`                         | [`actix-multipart`](https://crates.io/crates/actix-multipart) |
| `rust_decimal`    | Enables documenting types from `rust_decimal`                            | [`rust_decimal`](https://crates.io/crates/rust-decimal)       |
| `uuid`            | Enables documenting types from `uuid`                                    | [`uuid`](https://crates.io/crates/uuid)                       |
| `url`             | Enables documenting types from `url`                                     | [`url`](https://crates.io/crates/url)                         |
| `extras`          | Enables `chrono`, `multipart`, `rust_decimal`, `uuid` and `url` features | All from previous features                                    |

### What's next
- Handle schema for errors using ApiErrorComponent derive macro

### Alternatives

| Crate                                             | Key differences                                                                                                                                                                                                                                                                                                                               |
|---------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [`paperclip`](https://crates.io/crates/paperclip) | Paperclip is similar to this project but generate Swagger v2 documentation. Paperclip also provide a tool to generate rust code from a Swagger v2 document.                                                                                                                                                                                   |
| [`utoipa`](https://crates.io/crates/utoipa)       | Utoipa-actix integration rely on [actix web macros](https://docs.rs/actix-web-macros/latest/actix_web_macros/) for routing definition. At first, we planned on relying on utoipa for OAS types and schema derivation but for now [utoipa doesn't support generic struct the way we intended to](https://github.com/juhaku/utoipa/issues/703). |
| [`okapi`](https://crates.io/crates/okapi)         | Pretty similar, based on schemars as well (and maintained by the founder of schemars) but not integrated with actix.                                                                                                                                                                                                                          |

### About us

apistos is provided by [Netwo](https://www.netwo.io).

We use this crate for our internal needs and therefore are committed to its maintenance, however we cannot provide any additional guaranty. Use it at your own risks.

While we won't invest in any feature we don't need, we are open to accept any pull request you might propose.

We are a France based full-remote company operating in the telecom sector. If you are interested in learning more, feel free to visit [our career page](https://www.netwo.io/carriere).
