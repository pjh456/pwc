use crate::count::Accumulator;

#[derive(Default)]
pub(crate) struct CharAccumulator {
    counts: u64,
}

impl Accumulator for CharAccumulator {
    fn feed(&mut self, buf: &[u8]) {
        if let Ok(s) = std::str::from_utf8(buf) {
            self.counts += s.chars().count() as u64;
        }
    }
    fn count(&self) -> u64 {
        self.counts
    }
}
