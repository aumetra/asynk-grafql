//! Async-graphql integration with Axum

mod extract;
mod query;
mod response;
#[cfg(not(target_arch = "wasm32"))]
mod subscription;

pub use extract::{GraphQLBatchRequest, GraphQLRequest, rejection};
pub use query::GraphQL;
pub use response::GraphQLResponse;
#[cfg(not(target_arch = "wasm32"))]
pub use subscription::{GraphQLProtocol, GraphQLSubscription, GraphQLWebSocket};
