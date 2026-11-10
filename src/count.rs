mod accumulator;
mod byte;
mod char;
mod line;
mod max_len;
mod word;

use accumulator::Accumulator;
use byte::ByteAccumulator;
use char::CharAccumulator;
use line::LineAccumulator;
use max_len::MaxLenAccumulator;
use word::WordAccumulator;

use crate::model::Col;
use crate::model::Stats;
use std::io::{self, BufRead};

pub fn count<R: BufRead>(reader: &mut R, cols: &[Col]) -> io::Result<Stats> {
    let mut buf = Vec::new();

    let want_lines = cols.contains(&Col::Lines);
    let want_words = cols.contains(&Col::Words);
    let want_chars = cols.contains(&Col::Chars);
    let want_bytes = cols.contains(&Col::Bytes);
    let want_maxlen = cols.contains(&Col::MaxLineLen);

    let mut lines = LineAccumulator::default();
    let mut words = WordAccumulator::default();
    let mut chars = CharAccumulator::default();
    let mut bytes = ByteAccumulator::default();
    let mut maxlen = MaxLenAccumulator::default();

    loop {
        buf.clear();
        let n = reader.read_until(b'\n', &mut buf)?;
        if n == 0 {
            break; // EOF
        }

        if want_lines {
            lines.feed(&buf);
        }
        if want_words {
            words.feed(&buf);
        }
        if want_chars {
            chars.feed(&buf);
        }
        if want_bytes {
            bytes.feed(&buf);
        }
        if want_maxlen {
            maxlen.feed(&buf);
        }
    }

    Ok(Stats {
        lines: lines.count(),
        words: words.count(),
        chars: chars.count(),
        bytes: bytes.count(),
        max_line_len: maxlen.count(),
    })
}
