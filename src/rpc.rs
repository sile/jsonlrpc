use std::io::{Read, Write};

use crate::{JsonlStream, Request};

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

    /// RPC call.
    ///
    /// If the request is a notification, the response will be `Ok(None)`.
    pub fn call<T>(&mut self, request: &T) -> Result<Option<T::Response>, serde_json::Error>
    where
        T: Request,
    {
        self.stream.write_object(request)?;
        if request.is_notification() {
            return Ok(None);
        }

        let response = self.stream.read_object()?;
        Ok(Some(response))
    }

    /// Batch RPC call.
    ///
    /// If all requests are notifications, the response will be `Ok(Vec::new())`.
    pub fn batch_call<T>(&mut self, requests: &[T]) -> Result<Vec<T::Response>, serde_json::Error>
    where
        T: Request,
    {
        self.stream.write_object(&requests)?;

        if requests.iter().all(|r| r.is_notification()) {
            return Ok(Vec::new());
        }

        let responses = self.stream.read_object()?;
        Ok(responses)
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
