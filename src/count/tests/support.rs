use crate::count::accumulator::Accumulator;

pub(crate) fn assert_chunk_invariant<A>(data: &[u8], expected: u64)
where
    A: Accumulator + Default,
{
    for size in 1..=data.len().max(1) {
        let mut acc = A::default();
        for chunk in data.chunks(size) {
            acc.feed(chunk);
        }
        assert_eq!(acc.count(), expected, "chunk_size={size}, data={data:?}");
    }
}
