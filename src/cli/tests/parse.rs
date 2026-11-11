use crate::cli::{Cli, TotalWhen};
use clap::Parser;
use clap::error::ErrorKind;
use rstest::rstest;

fn parse(args: &[&str]) -> Cli {
    let mut argv = vec!["pwc"];
    argv.extend_from_slice(args);
    Cli::try_parse_from(argv).unwrap()
}

#[test]
fn defaults() {
    let cli = parse(&[]);

    assert!(!cli.lines);
    assert!(!cli.words);
    assert!(!cli.chars);
    assert!(!cli.bytes);
    assert!(!cli.max_line_len);
    assert_eq!(cli.total, TotalWhen::Auto);
    assert!(cli.files.is_empty());
}

#[rstest]
#[case::auto(&["--total=auto"], TotalWhen::Auto)]
#[case::always(&["--total=always"], TotalWhen::Always)]
#[case::only(&["--total=only"], TotalWhen::Only)]
#[case::never(&["--total=never"], TotalWhen::Never)]
fn parses_total(#[case] args: &[&str], #[case] want: TotalWhen) {
    assert_eq!(parse(args).total, want);
}

#[test]
fn parses_total_with_a_separate_value() {
    assert_eq!(parse(&["--total", "always"]).total, TotalWhen::Always);
}

#[test]
fn rejects_unknown_total_value() {
    let err = Cli::try_parse_from(["pwc", "--total=sometimes"]).unwrap_err();

    assert_eq!(err.kind(), ErrorKind::InvalidValue);
}

#[test]
fn rejects_unknown_flag() {
    let err = Cli::try_parse_from(["pwc", "--bogus"]).unwrap_err();

    assert_eq!(err.kind(), ErrorKind::UnknownArgument);
}

#[test]
fn rejects_unknown_short_flag() {
    let err = Cli::try_parse_from(["pwc", "-z"]).unwrap_err();

    assert_eq!(err.kind(), ErrorKind::UnknownArgument);
}

#[test]
fn double_dash_terminates_options() {
    let cli = parse(&["--", "-l"]);

    assert!(!cli.lines);
    assert_eq!(cli.files.len(), 1);
    assert_eq!(cli.files[0], std::path::PathBuf::from("-l"));
}

#[test]
fn help_is_displayed() {
    let err = Cli::try_parse_from(["pwc", "--help"]).unwrap_err();

    assert_eq!(err.kind(), ErrorKind::DisplayHelp);
}

#[test]
fn version_is_displayed() {
    let err = Cli::try_parse_from(["pwc", "--version"]).unwrap_err();

    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
}

#[test]
fn parses_all_short_flags_together() {
    let cli = parse(&["-lwmcL"]);

    assert!(cli.lines);
    assert!(cli.words);
    assert!(cli.chars);
    assert!(cli.bytes);
    assert!(cli.max_line_len);
}

#[test]
fn parses_all_long_flags() {
    let cli = parse(&[
        "--lines",
        "--words",
        "--chars",
        "--bytes",
        "--max-line-length",
    ]);

    assert!(cli.lines);
    assert!(cli.words);
    assert!(cli.chars);
    assert!(cli.bytes);
    assert!(cli.max_line_len);
}

#[test]
fn parses_combined_short_flags() {
    let cli = parse(&["-lw"]);

    assert!(cli.lines);
    assert!(cli.words);
    assert!(!cli.chars);
    assert!(!cli.bytes);
    assert!(!cli.max_line_len);
}

#[test]
fn collects_positional_files() {
    let cli = parse(&["a.txt", "b.txt"]);

    assert_eq!(cli.files.len(), 2);
    assert_eq!(cli.files[0], std::path::PathBuf::from("a.txt"));
    assert_eq!(cli.files[1], std::path::PathBuf::from("b.txt"));
}

#[test]
fn flags_and_files_can_mix() {
    let cli = parse(&["-l", "a.txt", "-", "b.txt"]);

    assert!(cli.lines);
    assert_eq!(cli.files.len(), 3);
}
