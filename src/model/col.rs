use crate::Stats;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Col {
    Lines,
    Words,
    Chars,
    Bytes,
    MaxLineLen,
}

impl Col {
    pub fn value(&self, stats: &Stats) -> u64 {
        match self {
            Col::Lines => stats.lines,
            Col::Words => stats.words,
            Col::Chars => stats.chars,
            Col::Bytes => stats.bytes,
            Col::MaxLineLen => stats.max_line_len,
        }
    }
}
