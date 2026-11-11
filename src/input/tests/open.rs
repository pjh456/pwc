use crate::input::Input;
use crate::input::inner::InputInner;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

#[test]
fn non_empty_file_is_mapped() {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(b"hello").unwrap();
    file.flush().unwrap();

    let input = Input::open(Some(file.path())).unwrap();

    assert!(matches!(&input.inner, InputInner::Mapped(_)));
    assert_eq!(input.as_slice(), Some(&b"hello"[..]));
}

#[test]
fn empty_file_is_empty_variant() {
    let file = NamedTempFile::new().unwrap();

    let input = Input::open(Some(file.path())).unwrap();

    assert!(matches!(&input.inner, InputInner::Empty));
    assert!(input.as_slice().is_none());
}

#[test]
fn empty_file_process_delivers_nothing() {
    let file = NamedTempFile::new().unwrap();
    let mut input = Input::open(Some(file.path())).unwrap();
    let mut calls = 0;

    input.process(&mut |_| calls += 1).unwrap();

    assert_eq!(calls, 0);
}

#[test]
fn missing_file_returns_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nope.txt");

    let err = Input::open(Some(&path)).err().unwrap();

    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn dash_opens_stdin_as_stream() {
    let input = Input::open(Some(Path::new("-"))).unwrap();

    assert!(matches!(&input.inner, InputInner::Stream(_)));
    assert!(input.as_slice().is_none());
}

#[test]
fn none_opens_stdin_as_stream() {
    let input = Input::open(None).unwrap();

    assert!(matches!(&input.inner, InputInner::Stream(_)));
    assert!(input.as_slice().is_none());
}
