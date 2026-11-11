use crate::input::Input;
use crate::input::inner::InputInner;
use std::io::{self, BufRead, Read, Write};
use tempfile::NamedTempFile;

struct ChunkedReader {
    data: Vec<u8>,
    pos: usize,
    step: usize,
}

impl ChunkedReader {
    fn new(data: &[u8], step: usize) -> Self {
        Self {
            data: data.to_vec(),
            pos: 0,
            step: step.max(1),
        }
    }
}

impl Read for ChunkedReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let chunk = self.fill_buf()?;
        let n = chunk.len().min(buf.len());
        buf[..n].copy_from_slice(&chunk[..n]);
        self.consume(n);
        Ok(n)
    }
}

impl BufRead for ChunkedReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        let end = self.pos.saturating_add(self.step).min(self.data.len());
        Ok(&self.data[self.pos..end])
    }

    fn consume(&mut self, n: usize) {
        self.pos = self.pos.saturating_add(n).min(self.data.len());
    }
}

struct FailingReader;

impl Read for FailingReader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::other("boom"))
    }
}

impl BufRead for FailingReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Err(io::Error::other("boom"))
    }

    fn consume(&mut self, _n: usize) {}
}

fn stream_input(data: &[u8], step: usize) -> Input {
    Input {
        inner: InputInner::Stream(Box::new(ChunkedReader::new(data, step))),
    }
}

fn collect_chunks(input: &mut Input) -> Vec<Vec<u8>> {
    let mut chunks = Vec::new();
    input
        .process(&mut |chunk| chunks.push(chunk.to_vec()))
        .unwrap();
    chunks
}

fn collect_chunks_with(input: &mut Input, size: usize) -> Vec<Vec<u8>> {
    let mut chunks = Vec::new();
    input
        .process_with_chunk_size(&mut |chunk| chunks.push(chunk.to_vec()), size)
        .unwrap();
    chunks
}

#[test]
fn stream_delivers_every_byte_once() {
    let data = b"abcdefghij";
    for step in [1, 2, 3, 4, 100] {
        let chunks = collect_chunks(&mut stream_input(data, step));
        let joined: Vec<u8> = chunks.concat();
        assert_eq!(joined, data, "step={step}");
    }
}

#[test]
fn stream_respects_short_reads() {
    let chunks = collect_chunks(&mut stream_input(b"abcdefg", 3));
    let sizes: Vec<usize> = chunks.iter().map(Vec::len).collect();
    assert_eq!(sizes, vec![3, 3, 1]);
}

#[test]
fn empty_stream_delivers_nothing() {
    assert!(collect_chunks(&mut stream_input(b"", 3)).is_empty());
}

#[test]
fn empty_variant_delivers_nothing() {
    let mut input = Input {
        inner: InputInner::Empty,
    };
    assert!(collect_chunks(&mut input).is_empty());
}

#[test]
fn stream_errors_are_propagated() {
    let mut input = Input {
        inner: InputInner::Stream(Box::new(FailingReader)),
    };
    assert!(input.process(&mut |_| {}).is_err());
}

#[test]
fn mapped_input_respects_chunk_size() {
    let data: Vec<u8> = (0..10u8).collect();
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(&data).unwrap();
    file.flush().unwrap();

    for size in [1, 3, 4, 10, 100] {
        let mut input = Input::open(Some(file.path())).unwrap();
        let chunks = collect_chunks_with(&mut input, size);
        let joined: Vec<u8> = chunks.concat();
        assert_eq!(joined, data, "size={size}");
        assert_eq!(chunks.len(), data.len().div_ceil(size), "size={size}");
        assert!(chunks.iter().all(|c| c.len() <= size), "size={size}");
    }
}

#[test]
fn mapped_input_uses_one_chunk_by_default_for_small_files() {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(b"hello").unwrap();
    file.flush().unwrap();
    let mut input = Input::open(Some(file.path())).unwrap();

    assert_eq!(collect_chunks(&mut input).len(), 1);
}
