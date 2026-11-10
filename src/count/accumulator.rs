pub trait Accumulator {
    fn feed(&mut self, buf: &[u8]);
    fn count(&self) -> u64;
}
