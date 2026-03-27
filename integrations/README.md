# Integrations for asynk-grafql

This directory provides various integrations for `asynk-grafql` to various crates in the ecosystem.

## Requirements for an HTTP integration

This is a list of criteria for HTTP integrations with `asynk-grafql` in order to make sure all
integrations are implemented consistently.

Integrations may provide additional functionality to better integrate with the specific library, but
they must all internally use the below functions.

- Conversion from HTTP library's request to `asynk_grafql::BatchRequest`:
	1. If the request is a `GET` request:
		1. Return the request's query parameters deserialized as an `asynk_grafql::Request`.
	1. If the request is a `POST` request:
		1. Get the request's `Content-Type` header.
		1. Call `asynk_grafql::http::receive_batch_body` on the request's body.
		1. Convert `ParseRequestError::PayloadTooLarge` to a 413 Payload Too Large response.
		1. Convert all other errors to a 400 Bad Request response.
	1. Otherwise return a 405 Method Not Allowed.
- Conversion from HTTP library's request to `asynk_grafql::Request`:
	1. Call the above function to convert the request to an `asynk_grafql::BatchRequest`.
	1. Call `BatchRequest::into_single` on the result.
	1. Convert all errors to a 400 Bad Request response.
- Conversion from `asynk_grafql::BatchResponse` to HTTP library's response:
	1. Create a 200 OK response.
	1. If the GraphQL response is ok, set the response's `Cache-Control` header to the response's
	   cache control value.
	1. Set the response's body to the GraphQL response serialized as JSON, also setting the
	   `Content-Type` header to `application/json`.
- GraphQL over websocket support:
	1. Create an `asynk_grafql::http:WebSocket` using `asynk_grafql::http::WebSocket::with_data`.
	1. Support the basics of the websocket protocol:
		- Respond to ping messages with pong messages.
		- Treat continuation messages identically to data messages.
	1. Stream all websocket messages that send data (bytes/text/continuations) to the
	   `asynk_grafql::http::WebSocket`.
	1. Convert all responses to websocket text responses.

## Integration Status

- **Poem**: Complete integration.
- **Actix-web**: Complete integration.
- **Rocket**: Missing websocket support (blocked on [support in Rocket itself](https://github.com/SergioBenitez/Rocket/issues/90)).
- **Warp**: Complete integration.
- **Axum**: Complete integration. 
