use crate::input::{Input, InputInner};

use std::io;

impl Input {
    pub fn process<F>(&mut self, mut f: F) -> io::Result<()>
    where
        F: FnMut(&[u8]),
    {
        match &mut self.inner {
            InputInner::Mapped(mmap) => {
                f(&mmap[..]);
                return Ok(());
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
