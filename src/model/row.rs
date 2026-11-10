use crate::{Label, Stats};

#[derive(Clone)]
pub struct Row<'a> {
    pub stats: Stats,
    pub label: Label<'a>,
}
