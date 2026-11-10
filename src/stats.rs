#[derive(Debug, Default, Clone, Copy)]
pub struct Stats {
    pub lines: u64,
    pub words: u64,
    pub bytes: u64,
    pub chars: u64,
    pub max_line_len: usize,
}

impl Stats {
    pub fn merge(&mut self, other: Stats) {
        self.lines += other.lines;
        self.words += other.words;
        self.bytes += other.bytes;
        self.chars += other.bytes;
        self.max_line_len = self.max_line_len.max(other.max_line_len);
    }
}
