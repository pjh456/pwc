use crate::model::{Col, Stats};
use rstest::rstest;

fn sample() -> Stats {
    Stats {
        lines: 1,
        words: 2,
        chars: 3,
        bytes: 4,
        max_line_len: 5,
    }
}

#[rstest]
#[case::lines(Col::Lines, 1)]
#[case::words(Col::Words, 2)]
#[case::chars(Col::Chars, 3)]
#[case::bytes(Col::Bytes, 4)]
#[case::max_line_len(Col::MaxLineLen, 5)]
fn maps_to_the_matching_field(#[case] col: Col, #[case] want: u64) {
    assert_eq!(col.value(&sample()), want);
}

#[test]
fn default_stats_are_zero_for_every_column() {
    let stats = Stats::default();
    for col in [
        Col::Lines,
        Col::Words,
        Col::Chars,
        Col::Bytes,
        Col::MaxLineLen,
    ] {
        assert_eq!(col.value(&stats), 0);
    }
}
