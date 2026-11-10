use crate::{Label, Row};

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub lines: u64,
    pub words: u64,
    pub chars: u64,
    pub bytes: u64,
    pub max_line_len: u64,
}

impl Stats {
    pub fn merge(&mut self, other: &Stats) {
        self.lines += other.lines;
        self.words += other.words;
        self.chars += other.chars;
        self.bytes += other.bytes;
        self.max_line_len = self.max_line_len.max(other.max_line_len);
    }

    pub fn with_label<'a>(self, label: Label<'a>) -> Row<'a> {
        Row { stats: self, label }
    }
}
