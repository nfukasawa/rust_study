use super::operations::Op;

pub struct Parser {
}

impl Parser {
    pub fn new() -> Self {
        Parser {  }
    }

    pub fn parse(&mut self, code: &[u8]) -> Vec<Op> {
        
        let l = code.len();
        let mut ops = Vec::with_capacity(l);
        let mut loop_stack = Vec::new();

        let mut pos = 0;
        while pos < l {
            match code[pos] {
                b'>' | b'<' => {
                    let mut v = 0;
                    while pos < l {
                        match code[pos] {
                            b'>' => v += 1,
                            b'<' => v -= 1,
                            b'+' | b'-' | b'.' | b',' | b'[' | b']' => {
                                pos -= 1;
                                break;
                            }
                            _ => (),
                        };
                        pos += 1;
                    }
                    if v != 0 {
                        ops.push(Op::MovPtr(v));
                    }
                }
                b'+' | b'-' => {
                    let mut v: i16 = 0;
                    while pos < l {
                        match code[pos] {
                            b'+' => v += 1,
                            b'-' => v -= 1,
                            b'>' | b'<' | b'.' | b',' | b'[' | b']' => {
                                pos -= 1;
                                break;
                            }
                            _ => (),
                        };
                        pos += 1;
                    }
                    if v != 0 {
                        ops.push(Op::AddVal(0, v));
                    }
                }
                b'.' => ops.push(Op::WriteVal(0)),
                b',' => ops.push(Op::ReadVal(0)),
                b'[' => {
                    loop_stack.push(ops.len());
                    ops.push(Op::LoopBegin(std::usize::MAX));
                }
                b']' => match loop_stack.pop() {
                    Some(pc) => match optimize_loop(&ops[pc + 1..ops.len()]) {
                        Some(mut ops0) => {
                            ops.truncate(pc);
                            ops.append(&mut ops0);
                        }
                        None => {
                            ops[pc] = Op::LoopBegin(ops.len());
                            ops.push(Op::LoopEnd(pc));
                        }
                    },
                    None => panic!("corresponding '[' not found: {}", pos),
                },
                _ => (),
            };
            pos += 1;
        }

        optimize_offsets(&ops)
    }
}

fn optimize_loop(ops: &[Op]) -> Option<Vec<Op>> {
    match ops {
        [] => None,

        // [-] [+]
        [Op::AddVal(_, 1)] | [Op::AddVal(_, -1)] => Some(vec![Op::ClearVal(0)]),

        // [>>] [<<]
        [Op::MovPtr(n)] => Some(vec![Op::SkipToZero(*n)]),

        // [>>+<<-] [<<+>>-] [>>-<<-] [<<->>-] [>>+<<+] [<<+>>+] [>>-<<+] [<<->>+]
        // [->>+<<] [-<<+>>] [->>-<<] [-<<->>] [+>>+<<] [+<<+>>] [+>>-<<] [+<<->>]
        [Op::MovPtr(n), Op::AddVal(_, mul), Op::MovPtr(m), Op::AddVal(_, s)]
        | [Op::AddVal(_, s), Op::MovPtr(n), Op::AddVal(_, mul), Op::MovPtr(m)]
            if *n == -*m && s.abs() == 1 =>
        {
            Some(vec![Op::MoveMulVal(0, *n, -*s * *mul)])
        }

        _ => optimize_move_mul_n(ops),
    }
}

// like [->+++>+++++++<<]
fn optimize_move_mul_n(ops: &[Op]) -> Option<Vec<Op>> {
    let ops0: &[Op];
    let s: i16;
    if let Some(Op::AddVal(_, n)) = ops.first() {
        s = *n;
        ops0 = &ops[1..];
    } else if let Some(Op::AddVal(_, n)) = ops.last() {
        s = *n;
        ops0 = &ops[..ops.len() - 1];
    } else {
        return None;
    }

    if s.abs() != 1 {
        return None;
    }

    let offset = match ops0.last() {
        Some(Op::MovPtr(n)) => *n,
        _ => return None,
    };

    let ops1 = &ops0[..ops0.len() - 1];

    let mut params = Vec::new();
    let mut pos = 0;
    for chunk in ops1.chunks(2) {
        match chunk {
            [Op::MovPtr(n), Op::AddVal(_, mul)] => {
                pos += *n;
                params.push((pos, -s * *mul));
            }
            _ => return None,
        }
    }

    if offset + pos == 0 {
        Some(vec![Op::MoveMulValN(0, params)])
    } else {
        None
    }
}

fn optimize_offsets(ops: &[Op]) -> Vec<Op> {
    let mut optimized = Vec::with_capacity(ops.len());
    let mut loop_stack = Vec::new();

    let mut offset = 0;
    for op in ops.iter() {
        match op {
            Op::MovPtr(n) => offset += *n,
            Op::AddVal(_, v) => optimized.push(Op::AddVal(offset, *v)),
            Op::WriteVal(_) => optimized.push(Op::WriteVal(offset)),
            Op::ReadVal(_) => optimized.push(Op::ReadVal(offset)),
            Op::LoopBegin(_) => {
                if offset != 0 {
                    optimized.push(Op::MovPtr(offset));
                    offset = 0;
                }
                loop_stack.push(optimized.len());
                optimized.push(Op::LoopBegin(std::usize::MAX));
            }
            Op::LoopEnd(_) => {
                if offset != 0 {
                    optimized.push(Op::MovPtr(offset));
                    offset = 0;
                }
                match loop_stack.pop() {
                    Some(pc) => {
                        optimized[pc] = Op::LoopBegin(optimized.len());
                        optimized.push(Op::LoopEnd(pc));
                    }
                    None => panic!("corresponding '[' not found"),
                }
            },
            Op::ClearVal(_) => optimized.push(Op::ClearVal(offset)),
            Op::MoveMulVal(_, n, mul) => optimized.push(Op::MoveMulVal(offset, *n, *mul)),
            Op::MoveMulValN(_, params) => optimized.push(Op::MoveMulValN(offset, params.clone())),
            Op::SkipToZero(n) => {
                if offset != 0 {
                    optimized.push(Op::MovPtr(offset));
                    offset = 0;
                }
                optimized.push(Op::SkipToZero(*n));
            }
        }
    }
    if offset != 0 {
        optimized.push(Op::MovPtr(offset));
    }
    optimized
}

#[allow(dead_code)]
impl std::string::ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::MovPtr(x) => {
                if *x > 0 {
                    ">".to_string()
                } else {
                    "<".to_string()
                }
            }
            Op::AddVal(_, x) => {
                if *x > 0 {
                    "+".to_string()
                } else {
                    "-".to_string()
                }
            }
            Op::WriteVal(_) => ".".to_string(),
            Op::ReadVal(_) => ",".to_string(),
            Op::LoopBegin(_) => '['.to_string(),
            Op::LoopEnd(_) => ']'.to_string(),
            Op::ClearVal(_) => "c".to_string(),
            Op::MoveMulVal(_, _, _) => "m".to_string(),
            Op::MoveMulValN(_, _) => "M".to_string(),
            Op::SkipToZero(_) => "s".to_string(),
        }
    }
}
#[allow(dead_code)]
fn to_str(ops: &[Op]) -> String {
    ops.iter().map(|op| op.to_string()).collect()
}
