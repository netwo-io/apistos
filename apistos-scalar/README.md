# Apistos Scalar &emsp; [![Documentation]][docs.rs] [![Latest Version]][crates.io] [![Build Status]][build] [![Deps Status]][deps.rs]

[docs.rs]: https://docs.rs/apistos-scalar/

[crates.io]: https://crates.io/crates/apistos-scalar

[build]: https://github.com/netwo-io/apistos/actions/workflows/build.yaml?branch=main

[Documentation]: https://img.shields.io/docsrs/apistos-scalar

[Latest Version]: https://img.shields.io/crates/v/apistos-scalar.svg

[Build Status]: https://github.com/netwo-io/apistos/actions/workflows/build.yaml/badge.svg?branch=main

[deps.rs]: https://deps.rs/crate/apistos-scalar

[Deps Status]: https://deps.rs/crate/apistos-scalar/latest/status.svg

Bridge between Apistos and [Scalar](https://scalar.com/) for actix.

This crate is exposed through Apistos `scalar` feature.

### Installation

```toml
[dependencies]
#schemars = "0.8"
# sadly we currently rely on a fork to fix multiple flatten for enums, related PR can be found here: https://github.com/GREsau/schemars/pull/264
schemars = { package = "apistos-schemars", version = "0.8" }
apistos = { version = "0.5", feature = ["scalar"] }
```

### About us

apistos is provided by [Netwo](https://www.netwo.io).

We use this crate for our internal needs and therefore are committed to its maintenance, however we cannot provide any
additional guaranty. Use it at your own risks.

While we won't invest in any feature we don't need, we are open to accept any pull request you might propose.

We are a France based full-remote company operating in the telecom industry. If you are interested in learning more,
feel free to visit [our career page](https://www.netwo.io/carriere).
