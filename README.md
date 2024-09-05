jsonlrpc
========

[![jsonlrpc](https://img.shields.io/crates/v/jsonlrpc.svg)](https://crates.io/crates/jsonlrpc)
[![Documentation](https://docs.rs/jsonlrpc/badge.svg)](https://docs.rs/jsonlrpc)
[![Actions Status](https://github.com/sile/jsonlrpc/workflows/CI/badge.svg)](https://github.com/sile/jsonlrpc/actions)
![License](https://img.shields.io/crates/l/jsonlrpc)

A [JSON-RPC 2.0] library that streams JSON objects in [JSON Lines] format.

[JSON-RPC 2.0]: https://www.jsonrpc.org/specification
[JSON Lines]: https://jsonlines.org/

Examples
--------

```rust
use std::net::TcpStream;
use jsonlrpc::{RpcClient, RequestId, RequestObject, ResponseObject, JsonRpcVersion};

// Connect to a JSON-RPC server.
let server_addr = /* ... */
let socket = TcpStream::connect(server_addr).expect("failed to connect to server");
let mut client = RpcClient::new(socket);

// Send a request to the server.
let request = RequestObject {
    jsonrpc: JsonRpcVersion::V2,
    id: Some(RequestId::Number(1)),
    method: "foo".to_string(),
    params: None,
};
let response = client.call(&request).expect("failed to RPC call");

// Check the response.
let Some(ResponseObject::Ok { result, id, .. }) = response else {
    panic!("expected ok response, got notification or err response")
};
assert_eq!(id, RequestId::Number(1));
```
