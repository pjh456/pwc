use crate::model::Stats;
use std::io::{self, BufRead};

pub fn count<R: BufRead>(reader: &mut R) -> io::Result<Stats> {
    let mut stats = Stats::default();
    let mut buf = Vec::new();
    let mut in_word = false;

    loop {
        buf.clear();
        let n = reader.read_until(b'\n', &mut buf)?;
        if n == 0 {
            break; // EOF
        }

        stats.bytes += n as u64;

        if buf.last() == Some(&b'\n') {
            stats.lines += 1;
        }

        for &b in &buf {
            if b.is_ascii_whitespace() {
                in_word = false;
            } else if !in_word {
                in_word = true;
                stats.words += 1;
            }
        }

        if let Ok(s) = std::str::from_utf8(&buf) {
            stats.chars += s.chars().count() as u64;
        }
    }

    Ok(stats)
}
