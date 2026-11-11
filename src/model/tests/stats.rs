use crate::model::{Label, Stats};
use std::path::Path;

fn sample() -> Stats {
    Stats {
        lines: 1,
        words: 2,
        chars: 3,
        bytes: 4,
        max_line_len: 5,
    }
}

#[test]
fn default_is_all_zero() {
    let stats = Stats::default();
    assert_eq!(stats.lines, 0);
    assert_eq!(stats.words, 0);
    assert_eq!(stats.chars, 0);
    assert_eq!(stats.bytes, 0);
    assert_eq!(stats.max_line_len, 0);
}

#[test]
fn merge_sums_counters_and_keeps_the_longest_line() {
    let mut a = Stats {
        lines: 1,
        words: 2,
        chars: 3,
        bytes: 4,
        max_line_len: 10,
    };
    let b = Stats {
        lines: 10,
        words: 20,
        chars: 30,
        bytes: 40,
        max_line_len: 7,
    };

    a.merge(&b);

    assert_eq!(a.lines, 11);
    assert_eq!(a.words, 22);
    assert_eq!(a.chars, 33);
    assert_eq!(a.bytes, 44);
    assert_eq!(a.max_line_len, 10);
}

#[test]
fn merge_takes_the_longer_line_from_the_other_side() {
    let mut a = Stats {
        max_line_len: 3,
        ..Stats::default()
    };
    let b = Stats {
        max_line_len: 8,
        ..Stats::default()
    };

    a.merge(&b);

    assert_eq!(a.max_line_len, 8);
}

#[test]
fn merge_with_default_is_identity() {
    let mut a = sample();

    a.merge(&Stats::default());

    assert_eq!(a.lines, 1);
    assert_eq!(a.words, 2);
    assert_eq!(a.chars, 3);
    assert_eq!(a.bytes, 4);
    assert_eq!(a.max_line_len, 5);
}

#[test]
fn merge_accumulates_over_many_inputs() {
    let mut total = Stats::default();

    for _ in 0..3 {
        total.merge(&sample());
    }

    assert_eq!(total.lines, 3);
    assert_eq!(total.words, 6);
    assert_eq!(total.chars, 9);
    assert_eq!(total.bytes, 12);
    assert_eq!(total.max_line_len, 5);
}

#[test]
fn with_label_keeps_stats_and_attaches_total() {
    let row = sample().with_label(Label::Total);

    assert_eq!(row.stats.lines, 1);
    assert_eq!(row.stats.bytes, 4);
    assert!(matches!(row.label, Label::Total));
}

#[test]
fn with_label_can_carry_stdin() {
    let row = Stats::default().with_label(Label::Stdin);

    assert_eq!(row.stats.bytes, 0);
    assert!(matches!(row.label, Label::Stdin));
}

#[test]
fn with_label_can_carry_a_file_path() {
    let path = Path::new("some/file.txt");
    let row = sample().with_label(Label::File(path));

    match row.label {
        Label::File(p) => assert_eq!(p, path),
        _ => panic!("expected a file label"),
    }
    assert_eq!(row.stats.max_line_len, 5);
}
