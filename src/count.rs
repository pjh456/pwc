mod accumulator;
mod byte;
mod char;
mod line;
mod max_len;
mod utf8;
mod word;

use accumulator::Accumulator;
use byte::ByteAccumulator;
use char::CharAccumulator;
use line::LineAccumulator;
use max_len::MaxLenAccumulator;
use utf8::Utf8Carry;
use word::WordAccumulator;

use crate::count::accumulator::Accumulators;
use crate::input::Input;
use crate::model::Col;
use crate::model::Stats;
use std::io;

pub fn count(inputs: &mut Input, cols: &[Col]) -> io::Result<Stats> {
    let mut accs = Accumulators::new(cols);

    inputs.process(&mut |chunk| {
        accs.feed_all(&chunk);
    })?;

    Ok(accs.to_stats())
}
