use std::io::{ErrorKind, Read, Write};

use serde::Deserialize;

#[derive(Debug)]
pub struct JsonlStream<S> {
    inner: S,
    read_buf: Vec<u8>,
    read_buf_offset: usize,
}

impl<S: Read + Write> JsonlStream<S> {
    pub fn new(inner: S) -> JsonlStream<S> {
        JsonlStream {
            inner,
            read_buf: vec![0; 1024],
            read_buf_offset: 0,
        }
    }

    pub fn read_item<T>(&mut self) -> Result<T, serde_json::Error>
    where
        T: for<'a> Deserialize<'a>,
    {
        loop {
            if self.read_buf_offset == self.read_buf.len() {
                self.read_buf.resize(self.read_buf.len() * 2, 0);
            }

            let read_size = self
                .inner
                .read(&mut self.read_buf[self.read_buf_offset..])
                .map_err(serde_json::Error::io)?;
            if read_size == 0 {
                return Err(serde_json::Error::io(ErrorKind::UnexpectedEof.into()));
            }

            let old_offset = self.read_buf_offset;
            self.read_buf_offset += read_size;

            for i in old_offset..self.read_buf_offset {
                if self.read_buf[i] == b'\n' {
                    let item = serde_json::from_slice(&self.read_buf[..i])?;
                    self.read_buf.copy_within(i + 1..self.read_buf_offset, 0);
                    self.read_buf_offset -= i + 1;
                    return Ok(item);
                }
            }
        }
    }

    pub fn inner(&self) -> &S {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    pub fn into_inner(self) -> S {
        self.inner
    }
}
