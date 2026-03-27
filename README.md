<div align="center">
<samp>

# asynk-grafql

**a high-performance graphql server library that's fully specification compliant**

</samp>

[Docs](https://docs.rs/asynk-grafql) • [GitHub repository](https://github.com/aumetra/asynk-grafql) • [Cargo package](https://crates.io/crates/asynk-grafql)

---

![ci status](https://github.com/aumetra/asynk-grafql/workflows/CI/badge.svg)
[![Unsafe Rust forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Crates.io version](https://img.shields.io/crates/v/asynk-grafql.svg)](https://crates.io/crates/asynk-grafql)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/asynk-grafql)
[![downloads](https://img.shields.io/crates/d/asynk-grafql.svg)](https://crates.io/crates/asynk-grafql)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/aumetra/asynk-grafql/compare)

_This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust._

</div>

## Example

```rs
use std::error::Error;

use asynk_grafql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use asynk_grafql::http::GraphiQLSource;
use axum::{Router, routing::get};

struct Query;

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        "partner"
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new().route(
        "/",
        get(|| async { axum::response::Html(GraphiQLSource::build().finish()) })
            .post_service(asynk_grafql::http::GraphQL::new(schema)),
    );
    println!("GraphiQL: http://localhost:8000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

## ⚠️Security

I strongly recommend limiting the [complexity and depth](https://docs.rs/asynk-grafql/latest/asynk_grafql/struct.SchemaBuilder.html) of queries in a production environment to avoid possible DDos attacks.

- [SchemaBuilder.limit_complexity](https://docs.rs/asynk-grafql/latest/asynk_grafql/struct.SchemaBuilder.html#method.limit_complexity)
- [SchemaBuilder.limit_depth](https://docs.rs/asynk-grafql/latest/asynk_grafql/struct.SchemaBuilder.html#method.limit_depth)
- [SchemaBuilder.limit_directives](https://docs.rs/asynk-grafql/latest/asynk_grafql/struct.SchemaBuilder.html#method.limit_directives)

## Features

- Fully supports async/await
- Type safety
- Rustfmt friendly (Procedural Macro)
- Custom scalars
- Minimal overhead
- Built-in [axum](https://crates.io/crates/axum) integration
- Upload files (Multipart request)
- Subscriptions (WebSocket transport)
- Custom extensions
- Error extensions
- Limit query complexity/depth
- Batch queries
- Apollo Persisted Queries
- Apollo Tracing extension
- Apollo Federation(v2)

> **Note**: Minimum supported Rust version: 1.86.0 or later

## Integrations

Integrations are what glue `asynk-grafql` with your web server.

- Axum — built-in (enable the `axum` feature, on by default)

## Crate features

This crate offers the following features. Most are not activated by default, except `tempfile`, `graphiql`, and `axum`:

| feature                        | enables                                                                                                                                                                                       |
| :----------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`apollo_tracing`**           | Enable the [Apollo tracing extension](https://docs.rs/asynk-grafql/latest/asynk_grafql/extensions/struct.ApolloTracing.html).                                                               |
| **`apollo_persisted_queries`** | Enable the [Apollo persisted queries extension](https://docs.rs/asynk-grafql/latest/asynk_grafql/extensions/apollo_persisted_queries/struct.ApolloPersistedQueries.html).                   |
| **`axum`**                     | Enable the built-in [Axum](https://crates.io/crates/axum) integration (enabled by default).                                                                                                  |
| **`dataloader`**               | Support [DataLoader](dataloader/struct.DataLoader.html).                                                                                                                                      |
| **`decimal`**                  | Integrate with the [`rust_decimal` crate](https://crates.io/crates/rust_decimal).                                                                                                             |
| **`graphiql`**                 | Enables the [GraphiQL IDE](https://github.com/graphql/graphiql) integration (enabled by default).                                                                                             |
| **`jiff`**                     | Integrate with the [`jiff` crate](https://crates.io/crates/jiff).                                                                                                                             |
| **`log`**                      | Enable the [Logger extension](https://docs.rs/asynk-grafql/latest/asynk_grafql/extensions/struct.Logger.html).                                                                              |
| **`raw_value`**                | Support raw values from [`serde_json`](https://crates.io/crates/serde_json)                                                                                                                   |
| **`secrecy`**                  | Integrate with the [`secrecy` crate](https://crates.io/crates/secrecy).                                                                                                                       |
| **`string_number`**            | Enable the [StringNumber](types/struct.StringNumber.html).                                                                                                                                    |
| **`time`**                     | Integrate with the [`time` crate](https://github.com/time-rs/time).                                                                                                                           |
| **`tracing`**                  | Enable the [Tracing extension](https://docs.rs/asynk-grafql/latest/asynk_grafql/extensions/struct.Tracing.html).                                                                            |
| **`tempfile`**                 | Save the uploaded content in the temporary file (enabled by default).                                                                                                                         |
| **`uuid`**                     | Integrate with the [`uuid` crate](https://crates.io/crates/uuid).                                                                                                                             |
| **`url`**                      | Integrate with the [`url` crate](https://crates.io/crates/url).                                                                                                                               |

## References

- [GraphQL](https://graphql.org)
- [GraphQL Multipart Request](https://github.com/jaydenseric/graphql-multipart-request-spec)
- [Multipart HTTP protocol for GraphQL subscriptions](https://www.apollographql.com/docs/router/executing-operations/subscription-multipart-protocol/)
- [GraphQL Cursor Connections Specification](https://facebook.github.io/relay/graphql/connections.htm)
- [GraphQL over WebSocket Protocol](https://github.com/apollographql/subscriptions-transport-ws/blob/master/PROTOCOL.md)
- [Apollo Tracing](https://github.com/apollographql/apollo-tracing)
- [Apollo Federation](https://www.apollographql.com/docs/apollo-server/federation/introduction)

## License

Licensed under either of

- Apache License, Version 2.0,
  ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.
