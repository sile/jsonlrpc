use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use crate::JsonlStream;

/// JSON-RPC client.
#[derive(Debug)]
pub struct RpcClient<S> {
    stream: JsonlStream<S>,
}

impl<S: Read + Write> RpcClient<S> {
    /// Makes a new [`RpcClient`] instance.
    pub fn new(stream: S) -> Self {
        Self {
            stream: JsonlStream::new(stream),
        }
    }

    /// RPC call (request).
    ///
    /// The `request` can be a batch (array) if it includes at least one non-notification request object.
    /// For a batch request that contains only notifications, use [`RpcClient::cast`] instead.
    pub fn call<REQ, RES>(&mut self, request: &REQ) -> Result<RES, serde_json::Error>
    where
        REQ: Serialize,
        RES: for<'de> Deserialize<'de>,
    {
        self.stream.write_value(request)?;
        let response = self.stream.read_value()?;
        Ok(response)
    }

    /// RPC call (notification).
    ///
    /// The `notification` can be a batch (array).
    pub fn cast<T>(&mut self, notification: &T) -> Result<(), serde_json::Error>
    where
        T: Serialize,
    {
        self.stream.write_value(notification)?;
        Ok(())
    }

    /// Returns a reference to the underlying JSONL stream.
    pub fn stream(&mut self) -> &JsonlStream<S> {
        &self.stream
    }

    /// Returns a mutable reference to the underlying JSONL stream.
    pub fn stream_mut(&mut self) -> &mut JsonlStream<S> {
        &mut self.stream
    }

    /// Consumes the [`RpcClient`] and returns the underlying JSONL stream.
    pub fn into_stream(self) -> JsonlStream<S> {
        self.stream
    }
}
