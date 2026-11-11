use crate::count::{Accumulator, Utf8Carry};

#[derive(Default)]
pub(crate) struct CharAccumulator {
    counts: u64,
    utf8: Utf8Carry,
}

impl Accumulator for CharAccumulator {
    fn feed(&mut self, buf: &[u8]) {
        let mut n = self.counts;
        self.utf8.for_each_char(buf, &mut |_| n += 1);
        self.counts = n;
    }

    fn count(&self) -> u64 {
        self.counts + self.utf8.pending_len() as u64
    }
}
