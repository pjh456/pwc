use crate::count::{Accumulator, Utf8Carry};

#[derive(Default)]
pub(crate) struct MaxLenAccumulator {
    max: u64,
    cur: u64,
    utf8: Utf8Carry,
}

impl Accumulator for MaxLenAccumulator {
    fn feed(&mut self, buf: &[u8]) {
        let (mut max, mut cur) = (self.max, self.cur);
        self.utf8.for_each_char(buf, &mut |c| {
            if c == '\n' {
                max = max.max(cur);
                cur = 0;
            } else {
                cur += 1;
            }
        });
        (self.max, self.cur) = (max, cur);
    }
    fn count(&self) -> u64 {
        self.max.max(self.cur + self.utf8.pending_len() as u64)
    }
}
