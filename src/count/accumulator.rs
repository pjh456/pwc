use crate::Col;
use crate::Stats;
use crate::count::ByteAccumulator;
use crate::count::CharAccumulator;
use crate::count::LineAccumulator;
use crate::count::MaxLenAccumulator;
use crate::count::WordAccumulator;

use std::option::Option;

pub trait Accumulator {
    /// Can be called in any situation,
    /// including empty, corssing lines, cutting UTF-8 code point inputs.
    /// Must reserve pending character.
    fn feed(&mut self, buf: &[u8]);
    /// Pending character is counted too.
    fn count(&self) -> u64;
}

pub struct Accumulators {
    lines: Option<LineAccumulator>,
    words: Option<WordAccumulator>,
    chars: Option<CharAccumulator>,
    bytes: Option<ByteAccumulator>,
    maxlen: Option<MaxLenAccumulator>,
}

impl Accumulators {
    pub fn new(cols: &[Col]) -> Self {
        let want_lines = cols.contains(&Col::Lines);
        let want_words = cols.contains(&Col::Words);
        let want_chars = cols.contains(&Col::Chars);
        let want_bytes = cols.contains(&Col::Bytes);
        let want_maxlen = cols.contains(&Col::MaxLineLen);

        Accumulators {
            lines: if want_lines {
                Some(LineAccumulator::default())
            } else {
                None
            },
            words: if want_words {
                Some(WordAccumulator::default())
            } else {
                None
            },
            chars: if want_chars {
                Some(CharAccumulator::default())
            } else {
                None
            },
            bytes: if want_bytes {
                Some(ByteAccumulator::default())
            } else {
                None
            },
            maxlen: if want_maxlen {
                Some(MaxLenAccumulator::default())
            } else {
                None
            },
        }
    }

    pub fn feed_all(&mut self, buf: &[u8]) {
        if let Some(l) = &mut self.lines {
            l.feed(buf);
        }
        if let Some(w) = &mut self.words {
            w.feed(buf);
        }
        if let Some(c) = &mut self.chars {
            c.feed(buf);
        }
        if let Some(b) = &mut self.bytes {
            b.feed(buf);
        }
        if let Some(m) = &mut self.maxlen {
            m.feed(buf);
        }
    }

    pub fn to_stats(&self) -> Stats {
        Stats {
            lines: match &self.lines {
                Some(l) => l.count(),
                None => 0,
            },
            words: match &self.words {
                Some(w) => w.count(),
                None => 0,
            },
            chars: match &self.chars {
                Some(c) => c.count(),
                None => 0,
            },
            bytes: match &self.bytes {
                Some(b) => b.count(),
                None => 0,
            },
            max_line_len: match &self.maxlen {
                Some(m) => m.count(),
                None => 0,
            },
        }
    }
}
