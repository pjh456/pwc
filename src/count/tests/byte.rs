use super::support::assert_chunk_invariant;
use crate::count::accumulator::Accumulator;
use crate::count::byte::ByteAccumulator;
use rstest::rstest;

#[rstest]
#[case::empty(b"", 0)]
#[case::ascii(b"abc", 3)]
#[case::utf8("你好".as_bytes(), 6)]
#[case::emoji("😀".as_bytes(), 4)]
#[case::newlines(b"a\nb\n", 4)]
#[case::invalid(b"\xFF\x00", 2)]
fn counts_every_byte_at_every_split(#[case] data: &[u8], #[case] want: u64) {
    assert_chunk_invariant::<ByteAccumulator>(data, want);
}

#[test]
fn empty_feed_does_not_change_count() {
    let mut acc = ByteAccumulator::default();
    acc.feed(b"abc");
    acc.feed(b"");
    assert_eq!(acc.count(), 3);
}
