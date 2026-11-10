use std::path::Path;

#[derive(Clone)]
pub enum Label<'a> {
    Stdin,
    File(&'a Path),
    Total,
}
