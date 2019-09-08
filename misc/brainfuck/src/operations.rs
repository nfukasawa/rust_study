#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Op {
    MovPtr(isize),
    AddVal(isize, i16),
    WriteVal(isize),
    ReadVal(isize),
    LoopBegin(usize),
    LoopEnd(usize),
    Loop(Vec<Op>),

    ClearVal(isize),
    MoveMulVal(isize, isize, i16),
    MoveMulValN(isize, Vec<(isize, i16)>),
    SkipToZero(isize),
}