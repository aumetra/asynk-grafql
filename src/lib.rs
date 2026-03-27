//! # A GraphQL server library implemented in Rust
//!
//! <div align="center">
//! <!-- CI -->
//! <img src="https://github.com/aumetra/asynk-grafql/workflows/CI/badge.svg" />
//! <!-- codecov -->
//! <img src="https://codecov.io/gh/aumetra/asynk-grafql/branch/master/graph/badge.svg" />
//! <!-- Crates version -->
//! <a href="https://crates.io/crates/asynk-grafql">
//! <img src="https://img.shields.io/crates/v/asynk-grafql.svg?style=flat-square"
//! alt="Crates.io version" />
//! </a>
//! <!-- Downloads -->
//! <a href="https://crates.io/crates/asynk-grafql">
//! <img src="https://img.shields.io/crates/d/asynk-grafql.svg?style=flat-square"
//! alt="Download" />
//! </a>
//! <!-- docs.rs docs -->
//! <a href="https://docs.rs/asynk-grafql">
//! <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
//! alt="docs.rs docs" />
//! </a>
//! <a href="https://github.com/rust-secure-code/safety-dance/">
//! <img src="https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square"
//! alt="Unsafe Rust forbidden" />
//! </a>
//! </div>
//!
//! ## Documentation
//!
//! * [Book](https://asynk-grafql.github.io/asynk-grafql/en/index.html)
//! * [中文文档](https://asynk-grafql.github.io/asynk-grafql/zh-CN/index.html)
//! * [Docs](https://docs.rs/asynk-grafql)
//! * [GitHub repository](https://github.com/aumetra/asynk-grafql)
//! * [Cargo package](https://crates.io/crates/asynk-grafql)
//! * Minimum supported Rust version: 1.56.1 or later
//!
//! ## Features
//!
//! * Fully supports async/await
//! * Type safety
//! * Rustfmt friendly (Procedural Macro)
//! * Custom scalars
//! * Minimal overhead
//! * Built-in [axum](https://crates.io/crates/axum) integration
//! * File upload (Multipart request)
//! * Subscriptions (WebSocket transport)
//! * Custom extensions
//! * Apollo Tracing extension
//! * Limit query complexity/depth
//! * Error Extensions
//! * Apollo Federation(v2)
//! * Batch Queries
//! * Apollo Persisted Queries
//!
//! ## Crate features
//!
//! This crate offers the following features, all of which are not activated by
//! default:
//!
//! | feature                        | enables                                                                                                                                                                                       |
//! |:-------------------------------|:----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | **`apollo_tracing`**           | Enable the [Apollo tracing extension](https://docs.rs/asynk-grafql/latest/asynk_grafql/extensions/struct.ApolloTracing.html).                                                               |
//! | **`apollo_persisted_queries`** | Enable the [Apollo persisted queries extension](https://docs.rs/asynk-grafql/latest/asynk_grafql/extensions/apollo_persisted_queries/struct.ApolloPersistedQueries.html).                   |
//! | **`dataloader`**               | Support [DataLoader](dataloader/struct.DataLoader.html).                                                                                                                                      |
//! | **`decimal`**                  | Integrate with the [`rust_decimal` crate](https://crates.io/crates/rust_decimal).                                                                                                             |
//! | **`axum`**                     | Enable the built-in [Axum](https://crates.io/crates/axum) integration (enabled by default).                                                                                                  |
//! | **`graphiql`**                 | Enables the [GraphiQL IDE](https://github.com/graphql/graphiql) integration (enabled by default).                                                                                             |
//! | **`jiff`**                     | Integrate with the [`jiff` crate](https://crates.io/crates/jiff).                                                                                                                             |
//! | **`log`**                      | Enable the [Logger extension](https://docs.rs/asynk-grafql/latest/asynk_grafql/extensions/struct.Logger.html).                                                                              |
//! | **`raw_value`**                | Support raw values from [`serde_json`](https://crates.io/crates/serde_json)                                                                                                                   |
//! | **`secrecy`**                  | Integrate with the [`secrecy` crate](https://crates.io/crates/secrecy).                                                                                                                       |
//! | **`string_number`**            | Enable the [StringNumber](types/struct.StringNumber.html).                                                                                                                                    |
//! | **`time`**                     | Integrate with the [`time` crate](https://github.com/time-rs/time).                                                                                                                           |
//! | **`tracing`**                  | Enable the [Tracing extension](https://docs.rs/asynk-grafql/latest/asynk_grafql/extensions/struct.Tracing.html).                                                                            |
//! | **`tempfile`**                 | Save the uploaded content in the temporary file (enabled by default).                                                                                                                         |
//! | **`uuid`**                     | Integrate with the [`uuid` crate](https://crates.io/crates/uuid).                                                                                                                             |
//! | **`url`**                      | Integrate with the [`url` crate](https://crates.io/crates/url).                                                                                                                               |
//!
//! ## Integrations
//!
//! * Axum — built-in (enable the `axum` feature, on by default)
//!
//! ## License
//!
//! Licensed under either of
//!
//! * Apache License, Version 2.0, (./LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license (./LICENSE-MIT or <http://opensource.org/licenses/MIT>) at
//!   your option.
//!
//! ## References
//!
//! * [GraphQL](https://graphql.org)
//! * [GraphQL Multipart Request](https://github.com/jaydenseric/graphql-multipart-request-spec)
//! * [GraphQL Cursor Connections Specification](https://facebook.github.io/relay/graphql/connections.htm)
//! * [GraphQL over WebSocket Protocol](https://github.com/apollographql/subscriptions-transport-ws/blob/master/PROTOCOL.md)
//! * [Apollo Tracing](https://github.com/apollographql/apollo-tracing)
//! * [Apollo Federation](https://www.apollographql.com/docs/apollo-server/federation/introduction)
//!
//! ## Examples
//!
//! All examples are in the [sub-repository](https://github.com/asynk-grafql/examples), located in the examples directory.
//!
//! **Run an example:**
//!
//! ```shell
//! git submodule update # update the examples repo
//! cd examples && cargo run --bin [name]
//! ```
//!
//! ## Benchmarks
//!
//! Ensure that there is no CPU-heavy process in background!
//!
//! ```shell script
//! cd benchmark
//! cargo bench
//! ```
//!
//! Now a HTML report is available at `benchmark/target/criterion/report`.

#![deny(clippy::all)]
// #![deny(clippy::pedantic)]
#![deny(clippy::inefficient_to_string)]
#![deny(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::if_not_else)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::needless_pass_by_value)]
#![deny(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::map_flatten)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::unused_self)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::implicit_hasher)]
// #![deny(clippy::nursery)]
#![allow(clippy::use_self)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::future_not_send)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::useless_let_if_seq)]
#![allow(clippy::uninlined_format_args)]
#![warn(missing_docs)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::upper_case_acronyms)]
#![recursion_limit = "256"]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod base;
mod custom_directive;
mod error;
mod executor;
mod guard;
mod look_ahead;
mod model;
mod request;
mod response;
mod schema;
mod subscription;
mod validation;

pub mod context;
#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
pub mod axum;
#[cfg(feature = "dataloader")]
#[cfg_attr(docsrs, doc(cfg(feature = "dataloader")))]
pub mod dataloader;
pub mod extensions;
pub mod http;
pub mod resolver_utils;
pub mod runtime;
pub mod types;
#[doc(hidden)]
pub mod validators;

#[doc(hidden)]
pub mod registry;

pub use asynk_grafql_parser as parser;
pub use asynk_grafql_value::{
    ConstValue as Value, DeserializerError, Extensions, Name, Number, SerializerError, Variables,
    from_value, to_value, value,
};
#[doc(hidden)]
pub use async_trait;
pub use base::{
    ComplexObject, Description, InputObjectType, InputType, InterfaceType, ObjectType,
    OneofObjectType, OutputType, TypeName, UnionType,
};
#[doc(hidden)]
pub use context::ContextSelectionSet;
pub use context::*;
pub use custom_directive::{CustomDirective, CustomDirectiveFactory, TypeDirective};
pub use error::{
    Error, ErrorExtensionValues, ErrorExtensions, InputValueError, InputValueResult,
    ParseRequestError, PathSegment, Result, ResultExt, ServerError, ServerResult,
};
pub use executor::Executor;
pub use extensions::ResolveFut;
#[doc(hidden)]
pub use futures_util;
pub use guard::{Guard, GuardExt};
#[doc(hidden)]
pub use indexmap;
pub use look_ahead::Lookahead;
#[doc(no_inline)]
pub use parser::{Pos, Positioned};
pub use registry::{CacheControl, SDLExportOptions};
pub use request::{BatchRequest, Request};
#[doc(no_inline)]
pub use resolver_utils::{ContainerType, EnumType, ScalarType};
pub use response::{BatchResponse, Response};
pub use schema::{IntrospectionMode, Schema, SchemaBuilder, SchemaEnv};
#[doc(hidden)]
pub use static_assertions_next;
pub use subscription::SubscriptionType;
pub use types::*;
pub use validation::{ValidationMode, ValidationResult, VisitorContext};
pub use validators::CustomValidator;

/// An alias of [asynk_grafql::Error](struct.Error.html). Present for backward
/// compatibility reasons.
pub type FieldError = Error;

/// An alias of [asynk_grafql::Result](type.Result.html). Present for backward
/// compatibility reasons.
pub type FieldResult<T> = Result<T>;

#[doc = include_str!("docs/complex_object.md")]
pub use asynk_grafql_derive::ComplexObject;
#[doc = include_str!("docs/description.md")]
pub use asynk_grafql_derive::Description;
#[doc = include_str!("docs/directive.md")]
pub use asynk_grafql_derive::Directive;
#[doc = include_str!("docs/enum.md")]
pub use asynk_grafql_derive::Enum;
#[doc = include_str!("docs/input_object.md")]
pub use asynk_grafql_derive::InputObject;
#[doc = include_str!("docs/interface.md")]
pub use asynk_grafql_derive::Interface;
#[doc = include_str!("docs/merged_object.md")]
pub use asynk_grafql_derive::MergedObject;
#[doc = include_str!("docs/merged_subscription.md")]
pub use asynk_grafql_derive::MergedSubscription;
#[doc = include_str!("docs/newtype.md")]
pub use asynk_grafql_derive::NewType;
#[doc = include_str!("docs/object.md")]
pub use asynk_grafql_derive::Object;
#[doc = include_str!("docs/oneof_object.md")]
pub use asynk_grafql_derive::OneofObject;
#[doc = include_str!("docs/scalar.md")]
pub use asynk_grafql_derive::Scalar;
#[doc = include_str!("docs/simple_object.md")]
pub use asynk_grafql_derive::SimpleObject;
#[doc = include_str!("docs/subscription.md")]
pub use asynk_grafql_derive::Subscription;
pub use asynk_grafql_derive::TypeDirective;
#[doc = include_str!("docs/union.md")]
pub use asynk_grafql_derive::Union;
