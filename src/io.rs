use std::io::{ErrorKind, Read, Write};

use serde::{Deserialize, Serialize};

/// JSONL stream.
#[derive(Debug)]
pub struct JsonlStream<S> {
    inner: S,
    read_buf: Vec<u8>,
    read_buf_offset: usize,
    write_buf: Vec<u8>,
    write_buf_offset: usize,
}

impl<S: Read + Write> JsonlStream<S> {
    /// Makes a new [`JsonlStream`] instance.
    pub fn new(inner: S) -> JsonlStream<S> {
        JsonlStream {
            inner,
            read_buf: vec![0; 1024],
            read_buf_offset: 0,
            write_buf: Vec::new(),
            write_buf_offset: 0,
        }
    }

    /// Reads a JSONL object from the stream.
    ///
    /// Note that if the inner stream is in non-blocking mode, this method may return
    /// [`ErrorKind::WouldBlock`] error.
    /// If it happens, you should retry this method after the stream becomes readable.
    pub fn read_object<T>(&mut self) -> Result<T, serde_json::Error>
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

    /// Writes a JSONL object to the stream.
    ///
    /// Note that if the inner stream is in non-blocking mode, this method may return
    /// [`ErrorKind::WouldBlock`] error.
    /// If it happens, you should retry by calling [`JsonlStream::flush()`] after the stream becomes writable.
    pub fn write_object<T>(&mut self, object: &T) -> Result<(), serde_json::Error>
    where
        T: Serialize,
    {
        serde_json::to_writer(&mut self.write_buf, object)?;
        self.write_buf.push(b'\n');
        self.flush()?;

        Ok(())
    }

    /// Writes all remaining data in the write buffer to the stream.
    ///
    /// You can use [`JsonlStream::write_buf()`] to check if there is any remaining data in the write buffer.
    ///
    /// As with [`JsonlStream::write_object()`], this method may return [`ErrorKind::WouldBlock`] error
    /// if the inner stream is in non-blocking mode.
    pub fn flush(&mut self) -> Result<(), serde_json::Error> {
        while self.write_buf_offset < self.write_buf.len() {
            let written_size = self
                .inner
                .write(&self.write_buf[self.write_buf_offset..])
                .map_err(serde_json::Error::io)?;
            if written_size == 0 {
                return Err(serde_json::Error::io(ErrorKind::WriteZero.into()));
            }
            self.write_buf_offset += written_size;
        }

        self.write_buf.clear();
        self.write_buf_offset = 0;

        Ok(())
    }

    /// Returns the incomplete JSON line in the read buffer.
    pub fn read_buf(&self) -> &[u8] {
        &self.read_buf[..self.read_buf_offset]
    }

    /// Returns the remaining data in the write buffer.
    pub fn write_buf(&self) -> &[u8] {
        &self.write_buf[self.write_buf_offset..]
    }

    /// Returns a reference to the inner stream.
    pub fn inner(&self) -> &S {
        &self.inner
    }

    /// Returns a mutable reference to the inner stream.
    pub fn inner_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    /// Consumes the [`JsonlStream`] and returns the inner stream.
    ///
    /// Note that any remaining data in the read and write buffers will be lost.
    pub fn into_inner(self) -> S {
        self.inner
    }
}
