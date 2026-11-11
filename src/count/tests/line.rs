use super::support::assert_chunk_invariant;
use crate::count::accumulator::Accumulator;
use crate::count::line::LineAccumulator;
use rstest::rstest;

#[rstest]
#[case::empty(b"", 0)]
#[case::no_newline(b"abc", 0)]
#[case::one_newline(b"abc\n", 1)]
#[case::two_lines(b"a\nb\n", 2)]
#[case::no_trailing_newline(b"a\nb", 1)]
#[case::blank_lines(b"\n\n", 2)]
#[case::only_newlines(b"\n\n\n", 3)]
#[case::crlf(b"a\r\nb\r\n", 2)]
fn counts_newlines_at_every_split(#[case] data: &[u8], #[case] want: u64) {
    assert_chunk_invariant::<LineAccumulator>(data, want);
}

#[test]
fn empty_feed_does_not_change_count() {
    let mut acc = LineAccumulator::default();
    acc.feed(b"a\n");
    acc.feed(b"");
    acc.feed(b"b\n");
    assert_eq!(acc.count(), 2);
}
