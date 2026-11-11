use crate::count::Accumulator;

#[derive(Default)]
pub(crate) struct LineAccumulator {
    counts: u64,
}

impl Accumulator for LineAccumulator {
    fn feed(&mut self, buf: &[u8]) {
        self.counts += buf.iter().filter(|&&b| b == b'\n').count() as u64;
    }
    fn count(&self) -> u64 {
        self.counts
    }
}
