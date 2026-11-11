use super::support::assert_chunk_invariant;
use crate::count::accumulator::Accumulator;
use crate::count::char::CharAccumulator;
use rstest::rstest;

#[rstest]
#[case::empty(b"", 0)]
#[case::ascii(b"abc", 3)]
#[case::newline(b"a\nb", 3)]
#[case::latin1("café".as_bytes(), 4)]
#[case::cjk("你好".as_bytes(), 2)]
#[case::emoji("😀".as_bytes(), 1)]
#[case::mixed("aé你😀\n".as_bytes(), 5)]
#[case::invalid_single(b"\xFF", 1)]
#[case::invalid_pair(b"\xE0\x80", 2)]
#[case::invalid_mixed(b"A\xFFB", 3)]
#[case::invalid_prefix(b"\xF0\x90\x80A", 4)]
fn counts_chars_at_every_split(#[case] data: &[u8], #[case] want: u64) {
    assert_chunk_invariant::<CharAccumulator>(data, want);
}

#[test]
fn counts_pending_bytes_at_eof() {
    let mut acc = CharAccumulator::default();
    acc.feed(&[0xE4]);
    assert_eq!(acc.count(), 1);
    acc.feed(&[0xBD]);
    assert_eq!(acc.count(), 2);
}

#[test]
fn completes_a_split_code_point() {
    let mut acc = CharAccumulator::default();
    acc.feed(&[0xE4]);
    acc.feed(&[0xBD]);
    acc.feed(&[0xA0]);
    assert_eq!(acc.count(), 1);
}

#[test]
fn empty_feed_does_not_change_count() {
    let mut acc = CharAccumulator::default();
    acc.feed("你".as_bytes());
    acc.feed(b"");
    assert_eq!(acc.count(), 1);
}
