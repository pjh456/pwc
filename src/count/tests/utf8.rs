use crate::count::utf8::Utf8Carry;
use proptest::prelude::*;
use rstest::rstest;

const REPL: char = char::REPLACEMENT_CHARACTER;

fn decode_all(data: &[u8], chunk_size: usize) -> Vec<char> {
    let mut carry = Utf8Carry::default();
    let mut out = Vec::new();

    for chunk in data.chunks(chunk_size) {
        carry.for_each_char(chunk, &mut |c| out.push(c));
    }

    out
}

fn assert_split_invariant(data: &[u8], want: &[char]) {
    for size in 1..=data.len().max(1) {
        assert_eq!(
            decode_all(data, size),
            want,
            "chunk_size={size}, data={data:?}"
        );
    }
}

#[rstest]
#[case::empty("")]
#[case::ascii("hello")]
#[case::two_byte("café")]
#[case::three_byte("你好")]
#[case::four_byte("😀")]
#[case::mixed("aé你😀\n")]
#[case::newlines("\n\n")]
fn valid_text_decodes_the_same_at_every_split(#[case] text: &str) {
    let want: Vec<char> = text.chars().collect();
    assert_split_invariant(text.as_bytes(), &want);
}

#[test]
fn long_mixed_text_in_tiny_chunks() {
    let text = "你好，世界！😀😀 café\n";
    let want: Vec<char> = text.chars().collect();
    assert_split_invariant(text.as_bytes(), &want);
}

#[test]
fn empty_chunk_is_a_noop() {
    let mut carry = Utf8Carry::default();
    let mut out = Vec::new();

    carry.for_each_char(b"", &mut |c| out.push(c));

    assert!(out.is_empty());
    assert_eq!(carry.pending_len(), 0);
}

#[test]
fn carry_holds_a_partial_code_point_across_feeds() {
    let mut carry = Utf8Carry::default();
    let mut out = Vec::new();

    carry.for_each_char(&[0xE4], &mut |c| out.push(c));
    assert_eq!(carry.pending_len(), 1);
    assert!(out.is_empty());

    carry.for_each_char(&[0xBD], &mut |c| out.push(c));
    assert_eq!(carry.pending_len(), 2);
    assert!(out.is_empty());

    carry.for_each_char(&[0xA0], &mut |c| out.push(c));
    assert_eq!(carry.pending_len(), 0);
    assert_eq!(out, vec!['你']);
}

#[test]
fn empty_chunk_does_not_disturb_the_carry() {
    let mut carry = Utf8Carry::default();
    let mut out = Vec::new();

    carry.for_each_char(&[0xF0], &mut |c| out.push(c));
    carry.for_each_char(b"", &mut |c| out.push(c));
    assert_eq!(carry.pending_len(), 1);

    carry.for_each_char(&[0x9F, 0x98, 0x80], &mut |c| out.push(c));

    assert_eq!(out, vec!['😀']);
    assert_eq!(carry.pending_len(), 0);
}

#[test]
fn incomplete_tail_is_not_emitted_during_feed() {
    let mut carry = Utf8Carry::default();
    let mut out = Vec::new();

    carry.for_each_char(&[0xE4, 0xBD], &mut |c| out.push(c));

    assert!(out.is_empty());
    assert_eq!(carry.pending_len(), 2);
}

#[rstest]
#[case::single_invalid(&[0xFF], &[REPL])]
#[case::lone_continuation(&[0x80], &[REPL])]
#[case::invalid_pair(&[0xE0, 0x80], &[REPL, REPL])]
#[case::valid_prefix_then_ascii(&[0xE0, 0xA0, b'A'], &[REPL, REPL, 'A'])]
#[case::bad_four_byte(&[0xF0, 0x90, 0x80, b'A'], &[REPL, REPL, REPL, 'A'])]
fn invalid_bytes_are_counted_one_by_one(#[case] data: &[u8], #[case] want: &[char]) {
    assert_split_invariant(data, want);
}

#[test]
fn resumes_normally_after_invalid_bytes() {
    assert_split_invariant(b"A\xFFB", &['A', REPL, 'B']);
}

#[test]
fn pending_len_never_exceeds_three() {
    let data = "😀你好café".as_bytes();

    for size in 1..=data.len() {
        let mut carry = Utf8Carry::default();
        for chunk in data.chunks(size) {
            carry.for_each_char(chunk, &mut |_| {});
            assert!(
                carry.pending_len() <= 3,
                "chunk_size={size}, pending={}",
                carry.pending_len()
            );
        }
    }
}

proptest! {
    #[test]
    fn decode_is_independent_of_chunking(text in any::<String>(), chunk_size in 1usize..64) {
        let want: Vec<char> = text.chars().collect();
        prop_assert_eq!(decode_all(text.as_bytes(), chunk_size), want);
    }
}
