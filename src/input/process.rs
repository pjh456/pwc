use crate::input::{Input, InputInner};

use std::io;

pub const DEFAULT_CHUNK_SIZE: usize = 64 * 1024;

impl Input {
    pub fn process<F>(&mut self, f: &mut F) -> io::Result<()>
    where
        F: FnMut(&[u8]),
    {
        self.process_with_chunk_size(f, DEFAULT_CHUNK_SIZE)
    }

    pub fn process_with_chunk_size<F>(&mut self, f: &mut F, chunk_size: usize) -> io::Result<()>
    where
        F: FnMut(&[u8]),
    {
        debug_assert!(chunk_size > 0);

        match &mut self.inner {
            InputInner::Mapped(mmap) => {
                for chunk in mmap[..].chunks(chunk_size) {
                    f(chunk);
                }
                Ok(())
            }
            InputInner::Stream(reader) => loop {
                let buf = reader.fill_buf()?;
                if buf.is_empty() {
                    return Ok(());
                }
                f(buf);
                let n = buf.len();
                reader.consume(n);
            },
            InputInner::Empty => Ok(()),
        }
    }
}
