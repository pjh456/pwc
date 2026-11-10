use memmap2::Mmap;
use std::io::BufRead;

pub(crate) enum InputInner {
    Mapped(Mmap),
    Stream(Box<dyn BufRead>),
    Empty,
}

pub struct Input {
    pub(crate) inner: InputInner,
}
