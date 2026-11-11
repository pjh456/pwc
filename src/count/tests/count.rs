use crate::Col;
use crate::Stats;
use crate::count::count as count_stats;
use crate::input::Input;
use std::io::Write;
use tempfile::NamedTempFile;

const ALL: [Col; 5] = [
    Col::Lines,
    Col::Words,
    Col::Chars,
    Col::Bytes,
    Col::MaxLineLen,
];

fn stats_of(data: &[u8], cols: &[Col]) -> Stats {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(data).unwrap();
    file.flush().unwrap();
    let mut input = Input::open(Some(file.path())).unwrap();
    count_stats(&mut input, cols).unwrap()
}

fn assert_zero(stats: &Stats) {
    assert_eq!(stats.lines, 0);
    assert_eq!(stats.words, 0);
    assert_eq!(stats.chars, 0);
    assert_eq!(stats.bytes, 0);
    assert_eq!(stats.max_line_len, 0);
}

#[test]
fn empty_input_counts_zero() {
    assert_zero(&stats_of(b"", &ALL));
}

#[test]
fn ascii_input() {
    let stats = stats_of(b"a b\nc\n", &ALL);
    assert_eq!(stats.lines, 2);
    assert_eq!(stats.words, 3);
    assert_eq!(stats.chars, 6);
    assert_eq!(stats.bytes, 6);
    assert_eq!(stats.max_line_len, 3);
}

#[test]
fn utf8_input() {
    let stats = stats_of("你好\n".as_bytes(), &ALL);
    assert_eq!(stats.lines, 1);
    assert_eq!(stats.words, 1);
    assert_eq!(stats.chars, 3);
    assert_eq!(stats.bytes, 7);
    assert_eq!(stats.max_line_len, 2);
}

#[test]
fn invalid_utf8_input() {
    let stats = stats_of(b"a\xFFb\n", &ALL);
    assert_eq!(stats.lines, 1);
    assert_eq!(stats.words, 1);
    assert_eq!(stats.chars, 4);
    assert_eq!(stats.bytes, 4);
    assert_eq!(stats.max_line_len, 3);
}

#[test]
fn only_requested_columns_are_reported() {
    let stats = stats_of("你好".as_bytes(), &[Col::Chars]);
    assert_eq!(stats.chars, 2);
    assert_eq!(stats.lines, 0);
    assert_eq!(stats.words, 0);
    assert_eq!(stats.bytes, 0);
    assert_eq!(stats.max_line_len, 0);
}
