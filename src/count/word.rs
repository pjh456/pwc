use crate::count::Accumulator;

#[derive(Default)]
pub(crate) struct WordAccumulator {
    counts: u64,
    in_word: bool,
}

impl Accumulator for WordAccumulator {
    fn feed(&mut self, buf: &[u8]) {
        for &b in buf {
            if b.is_ascii_whitespace() {
                self.in_word = false;
            } else if !self.in_word {
                self.in_word = true;
                self.counts += 1;
            }
        }
    }
    fn count(&self) -> u64 {
        self.counts
    }
}
