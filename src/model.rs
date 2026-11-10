use std::path::Path;

#[derive(Debug, Default, Clone, Copy)]
pub struct Stats {
    pub lines: u64,
    pub words: u64,
    pub chars: u64,
    pub bytes: u64,
    pub max_line_len: u64,
}

#[derive(Clone)]
pub enum Label<'a> {
    Stdin,
    File(&'a Path),
    Total,
}

#[derive(Clone)]
pub struct Row<'a> {
    pub stats: Stats,
    pub label: Label<'a>,
}

#[derive(Clone)]
pub enum Col {
    Lines,
    Words,
    Chars,
    Bytes,
    MaxLineLen,
}

impl Stats {
    pub fn merge(&mut self, other: Stats) {
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

impl Col {
    pub fn value(&self, stats: &Stats) -> u64 {
        match self {
            Col::Lines => stats.lines,
            Col::Words => stats.words,
            Col::Chars => stats.chars,
            Col::Bytes => stats.bytes,
            Col::MaxLineLen => stats.max_line_len as u64,
        }
    }
}
