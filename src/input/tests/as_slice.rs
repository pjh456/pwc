use crate::input::Input;
use crate::input::inner::InputInner;
use std::io::{Cursor, Write};
use tempfile::NamedTempFile;

#[test]
fn mapped_input_exposes_slice() {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(b"abc").unwrap();
    file.flush().unwrap();
    let input = Input::open(Some(file.path())).unwrap();

    assert_eq!(input.as_slice(), Some(&b"abc"[..]));
}

#[test]
fn stream_input_has_no_slice() {
    let input = Input {
        inner: InputInner::Stream(Box::new(Cursor::new(vec![1, 2, 3]))),
    };

    assert!(input.as_slice().is_none());
}

#[test]
fn empty_input_has_no_slice() {
    let input = Input {
        inner: InputInner::Empty,
    };

    assert!(input.as_slice().is_none());
}
