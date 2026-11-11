use crate::Stats;
use crate::count::accumulator::Accumulators;
use crate::model::Col;

fn assert_zero(stats: &Stats) {
    assert_eq!(stats.lines, 0);
    assert_eq!(stats.words, 0);
    assert_eq!(stats.chars, 0);
    assert_eq!(stats.bytes, 0);
    assert_eq!(stats.max_line_len, 0);
}

#[test]
fn no_columns_means_no_counts() {
    let mut accs = Accumulators::new(&[]);
    accs.feed_all(b"a b\nc\n");
    assert_zero(&accs.to_stats());
}

#[test]
fn only_requested_columns_are_counted() {
    let mut accs = Accumulators::new(&[Col::Lines]);
    accs.feed_all(b"a b\nc\n");
    let stats = accs.to_stats();
    assert_eq!(stats.lines, 2);
    assert_eq!(stats.words, 0);
    assert_eq!(stats.chars, 0);
    assert_eq!(stats.bytes, 0);
    assert_eq!(stats.max_line_len, 0);
}

#[test]
fn all_columns_together() {
    let mut accs = Accumulators::new(&[
        Col::Lines,
        Col::Words,
        Col::Chars,
        Col::Bytes,
        Col::MaxLineLen,
    ]);
    accs.feed_all("aé\n".as_bytes());
    let stats = accs.to_stats();
    assert_eq!(stats.lines, 1);
    assert_eq!(stats.words, 1);
    assert_eq!(stats.chars, 3);
    assert_eq!(stats.bytes, 4);
    assert_eq!(stats.max_line_len, 2);
}

#[test]
fn stats_are_zero_before_any_feed() {
    let accs = Accumulators::new(&[Col::Lines, Col::Bytes]);
    assert_zero(&accs.to_stats());
}

#[test]
fn empty_feed_does_not_change_stats() {
    let mut accs = Accumulators::new(&[Col::Bytes]);
    accs.feed_all(b"abc");
    accs.feed_all(b"");
    assert_eq!(accs.to_stats().bytes, 3);
}

#[test]
fn duplicate_columns_are_harmless() {
    let mut accs = Accumulators::new(&[Col::Lines, Col::Lines]);
    accs.feed_all(b"a\nb\n");
    assert_eq!(accs.to_stats().lines, 2);
}
