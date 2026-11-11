use super::support::assert_chunk_invariant;
use crate::count::accumulator::Accumulator;
use crate::count::max_len::MaxLenAccumulator;
use rstest::rstest;

#[rstest]
#[case::empty(b"", 0)]
#[case::single_line(b"abc", 3)]
#[case::trailing_newline(b"abc\n", 3)]
#[case::tail_without_newline(b"a\nabcd\nb", 4)]
#[case::longest_in_middle(b"a\nbbbb\ncc", 4)]
#[case::blank_lines(b"\n\n", 0)]
#[case::cjk("你好\n".as_bytes(), 2)]
#[case::cjk_tail("a\n你好\nbb".as_bytes(), 2)]
#[case::crlf(b"a\r\nbb\r\n", 3)]
#[case::tab(b"a\tb", 3)]
#[case::invalid_single(b"\xFF", 1)]
#[case::invalid_mixed(b"A\xFFB", 3)]
fn reports_longest_line_at_every_split(#[case] data: &[u8], #[case] want: u64) {
    assert_chunk_invariant::<MaxLenAccumulator>(data, want);
}

#[test]
fn newline_can_be_split_from_its_line() {
    let mut acc = MaxLenAccumulator::default();
    acc.feed(b"ab");
    acc.feed(b"\ncd");
    assert_eq!(acc.count(), 2);
}

#[test]
fn pending_bytes_count_toward_current_line() {
    let mut acc = MaxLenAccumulator::default();
    acc.feed(&[0xE4, 0xBD]);
    assert_eq!(acc.count(), 2);
}

#[test]
fn empty_feed_does_not_change_count() {
    let mut acc = MaxLenAccumulator::default();
    acc.feed(b"abc\n");
    acc.feed(b"");
    acc.feed(b"de");
    assert_eq!(acc.count(), 3);
}
