use crate::input::{Input, inner::InputInner};

impl Input {
    pub fn as_slice(&self) -> Option<&[u8]> {
        match &self.inner {
            InputInner::Mapped(mmap) => Some(&mmap[..]),
            _ => None,
        }
    }
}
