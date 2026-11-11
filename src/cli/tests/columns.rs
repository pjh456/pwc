use crate::Col;
use crate::cli::Cli;
use clap::Parser;
use clap::error::ErrorKind;
use rstest::rstest;

fn parse(args: &[&str]) -> Cli {
    let mut argv = vec!["pwc"];
    argv.extend_from_slice(args);
    Cli::try_parse_from(argv).unwrap()
}

#[test]
fn defaults_to_lines_words_bytes() {
    assert_eq!(
        parse(&[]).columns(),
        vec![Col::Lines, Col::Words, Col::Bytes]
    );
}

#[rstest]
#[case::lines(&["-l"], &[Col::Lines])]
#[case::words(&["-w"], &[Col::Words])]
#[case::chars(&["-m"], &[Col::Chars])]
#[case::bytes(&["-c"], &[Col::Bytes])]
#[case::max_line_len(&["-L"], &[Col::MaxLineLen])]
fn single_flag_selects_only_that_column(#[case] args: &[&str], #[case] want: &[Col]) {
    assert_eq!(parse(args).columns(), want);
}

#[test]
fn order_follows_the_struct_not_the_command_line() {
    assert_eq!(parse(&["-c", "-l"]).columns(), vec![Col::Lines, Col::Bytes]);
}

#[test]
fn all_flags_in_struct_order() {
    assert_eq!(
        parse(&["-L", "-m", "-w", "-l", "-c"]).columns(),
        vec![
            Col::Lines,
            Col::Words,
            Col::Chars,
            Col::Bytes,
            Col::MaxLineLen
        ]
    );
}

#[test]
fn duplicate_flags_are_rejected() {
    let err = Cli::try_parse_from(["pwc", "-l", "-l"]).unwrap_err();

    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn clustered_duplicate_flags_are_rejected() {
    let err = Cli::try_parse_from(["pwc", "-ll"]).unwrap_err();

    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn total_does_not_affect_columns() {
    assert_eq!(
        parse(&["--total=only"]).columns(),
        vec![Col::Lines, Col::Words, Col::Bytes]
    );
}
