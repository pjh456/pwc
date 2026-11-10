use crate::count::Accumulator;

#[derive(Default)]
pub(crate) struct LineAccumulator {
    counts: u64,
}

impl Accumulator for LineAccumulator {
    fn feed(&mut self, _buf: &[u8]) {
        self.counts += 1;
    }
    fn count(&self) -> u64 {
        self.counts
    }
}
