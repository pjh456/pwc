use super::support::assert_chunk_invariant;
use crate::count::accumulator::Accumulator;
use crate::count::word::WordAccumulator;
use rstest::rstest;

#[rstest]
#[case::empty(b"", 0)]
#[case::spaces_only(b"   ", 0)]
#[case::single(b"a", 1)]
#[case::two(b"a b", 2)]
#[case::padded(b"  a  b  ", 2)]
#[case::newline(b"a\nb", 2)]
#[case::all_whitespace(b"\t\n\r ", 0)]
#[case::form_feed(b"a\x0cb", 2)]
#[case::vertical_tab(b"a\x0bb", 1)]
#[case::punctuation(b"a-b.c,d", 1)]
#[case::utf8("你好".as_bytes(), 1)]
#[case::mixed(b"a1 b2\tc3\nd4", 4)]
fn counts_words_at_every_split(#[case] data: &[u8], #[case] want: u64) {
    assert_chunk_invariant::<WordAccumulator>(data, want);
}

#[test]
fn empty_feed_does_not_change_count() {
    let mut acc = WordAccumulator::default();
    acc.feed(b"a b");
    acc.feed(b"");
    assert_eq!(acc.count(), 2);
}
