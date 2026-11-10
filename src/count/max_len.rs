use crate::count::Accumulator;

#[derive(Default)]
pub(crate) struct MaxLenAccumulator {
    counts: u64,
}

impl Accumulator for MaxLenAccumulator {
    fn feed(&mut self, buf: &[u8]) {
        self.counts = self.counts.max(buf.len() as u64);
    }
    fn count(&self) -> u64 {
        self.counts
    }
}
