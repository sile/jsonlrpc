use std::io::{Read, Write};

use crate::{JsonlStream, Request};

#[derive(Debug)]
pub struct RpcClient<S> {
    stream: JsonlStream<S>,
}

impl<S: Read + Write> RpcClient<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream: JsonlStream::new(stream),
        }
    }

    pub fn call<T>(&mut self, request: &T) -> Result<Option<T::Response>, serde_json::Error>
    where
        T: Request,
    {
        self.stream.write_item(request)?;
        if request.is_notification() {
            return Ok(None);
        }

        let response = self.stream.read_item()?;
        Ok(Some(response))
    }

    pub fn batch_call<T>(&mut self, requests: &[T]) -> Result<Vec<T::Response>, serde_json::Error>
    where
        T: Request,
    {
        self.stream.write_item(&requests)?;

        if requests.iter().all(|r| r.is_notification()) {
            return Ok(Vec::new());
        }

        let responses = self.stream.read_item()?;
        Ok(responses)
    }
}
