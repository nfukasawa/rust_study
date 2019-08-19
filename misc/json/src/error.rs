#[derive(Clone, Debug, PartialEq)]
pub struct Err {} // TODO

impl Err {
    // TODO: embed error info: cause, line, position, ...
    pub fn new() -> Self {
        Err {}
    }
}
