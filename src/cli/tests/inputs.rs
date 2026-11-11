use crate::cli::Cli;
use clap::Parser;
use std::path::Path;

fn parse(args: &[&str]) -> Cli {
    let mut argv = vec!["pwc"];
    argv.extend_from_slice(args);
    Cli::try_parse_from(argv).unwrap()
}

#[test]
fn no_files_means_stdin() {
    assert_eq!(parse(&[]).inputs(), vec![None]);
}

#[test]
fn single_file() {
    assert_eq!(parse(&["a.txt"]).inputs(), vec![Some(Path::new("a.txt"))]);
}

#[test]
fn dash_means_stdin() {
    assert_eq!(parse(&["-"]).inputs(), vec![None]);
}

#[test]
fn mixes_files_and_stdin_in_order() {
    assert_eq!(
        parse(&["a", "-", "b"]).inputs(),
        vec![Some(Path::new("a")), None, Some(Path::new("b"))]
    );
}

#[test]
fn multiple_dashes_are_all_stdin() {
    assert_eq!(parse(&["-", "-"]).inputs(), vec![None, None]);
}

#[test]
fn path_that_looks_like_dash_is_still_a_file() {
    assert_eq!(parse(&["./-"]).inputs(), vec![Some(Path::new("./-"))]);
}
