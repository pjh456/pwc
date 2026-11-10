use crate::count::Accumulator;

#[derive(Default)]
pub(crate) struct ByteAccumulator {
    counts: u64,
}

impl Accumulator for ByteAccumulator {
    fn feed(&mut self, buf: &[u8]) {
        self.counts += buf.len() as u64;
    }
    fn count(&self) -> u64 {
        self.counts
    }
}
