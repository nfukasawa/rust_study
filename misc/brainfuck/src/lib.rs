use std::io;

pub struct Interpreter<R: io::Read, W: io::Write> {
    input: R,
    output: W,
}

impl<R: io::Read, W: io::Write> Interpreter<R, W> {
    pub fn new(input: R, output: W) -> Self {
        Interpreter {
            input: input,
            output: output,
        }
    }

    pub fn interpret(&mut self, code: &[u8]) {
        let ops = opertions(code);
        exec(&ops, &mut self.input, &mut self.output);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Op {
    IncPtr(usize),
    DecPtr(usize),
    IncVal(u8),
    DecVal(u8),
    WriteVal,
    ReadVal,
    LoopBegin(usize),
    LoopEnd(usize),

    OptSetValZero,
    OptAddValRight(usize),
    OptAddValLeft(usize),
    OptSubValRight(usize),
    OptSubValLeft(usize),
    OptSearchZeroRight(usize),
    OptSearchZeroLeft(usize),
}

fn opertions(code: &[u8]) -> Vec<Op> {
    let l = code.len();
    let mut ops = Vec::with_capacity(l);
    let mut loop_stack = Vec::new();

    let mut pos = 0;
    while pos < l {
        match code[pos] {
            b'>' | b'<' => {
                let mut v: isize = 0;
                while pos < l {
                    v += match code[pos] {
                        b'>' => 1,
                        b'<' => -1,
                        _ => {
                            pos -= 1;
                            break;
                        }
                    };
                    pos += 1;
                }
                if v > 0 {
                    ops.push(Op::IncPtr(v as usize));
                } else if v < 0 {
                    ops.push(Op::DecPtr(-v as usize));
                }
            }
            b'+' | b'-' => {
                let mut v: i64 = 0;
                while pos < l {
                    v += match code[pos] {
                        b'+' => 1,
                        b'-' => -1,
                        _ => {
                            pos -= 1;
                            break;
                        }
                    };
                    pos += 1;
                }
                if v > 0 {
                    ops.push(Op::IncVal(v as u8));
                } else if v < 0 {
                    ops.push(Op::DecVal(-v as u8));
                }
            }
            b'.' => ops.push(Op::WriteVal),
            b',' => ops.push(Op::ReadVal),
            b'[' => {
                loop_stack.push(ops.len());
                ops.push(Op::LoopBegin(0));
            }
            b']' => {
                if let Some(i) = loop_stack.pop() {
                    if let Some(mut ops0) = optimize_loop(&ops[i + 1..ops.len()]) {
                        ops.truncate(i);
                        ops.append(&mut ops0);
                    } else {
                        ops[i] = Op::LoopBegin(ops.len());
                        ops.push(Op::LoopEnd(i));
                    }
                } else {
                    panic!("corresponding '[' not found: {}", pos);
                }
            }
            _ => (),
        };
        pos += 1;
    }
    ops
}

fn optimize_loop(ops: &[Op]) -> Option<Vec<Op>> {
    match ops {
        // [-]
        [Op::DecVal(1)] => Some(vec![Op::OptSetValZero]),

        // [>>>+<<<-]
        [Op::IncPtr(n), Op::IncVal(1), Op::DecPtr(m), Op::DecVal(1)] if n == m => {
            Some(vec![Op::OptAddValRight(*n)])
        }
        // [->>>+<<<]
        [Op::DecVal(1), Op::IncPtr(n), Op::IncVal(1), Op::DecPtr(m)] if n == m => {
            Some(vec![Op::OptAddValRight(*n)])
        }

        // [<<<+>>>-]
        [Op::DecPtr(n), Op::IncVal(1), Op::IncPtr(m), Op::DecVal(1)] if n == m => {
            Some(vec![Op::OptAddValLeft(*n)])
        }
        // [-<<<+>>>]
        [Op::DecVal(1), Op::DecPtr(n), Op::IncVal(1), Op::IncPtr(m)] if n == m => {
            Some(vec![Op::OptAddValLeft(*n)])
        }

        // [>>>-<<<-]
        [Op::IncPtr(n), Op::DecVal(1), Op::DecPtr(m), Op::DecVal(1)] if n == m => {
            Some(vec![Op::OptSubValRight(*n)])
        }
        // [->>>-<<<]
        [Op::DecVal(1), Op::IncPtr(n), Op::DecVal(1), Op::DecPtr(m)] if n == m => {
            Some(vec![Op::OptSubValRight(*n)])
        }

        // [<<<->>>-]
        [Op::DecPtr(n), Op::IncVal(1), Op::DecPtr(m), Op::DecVal(1)] if n == m => {
            Some(vec![Op::OptSubValLeft(*n)])
        }
        // [-<<<->>>]
        [Op::DecVal(1), Op::DecPtr(n), Op::DecVal(1), Op::IncPtr(m)] if n == m => {
            Some(vec![Op::OptSubValLeft(*n)])
        }

        // [>>>]
        [Op::IncPtr(n)] => Some(vec![Op::OptSearchZeroRight(*n)]),

        // [<<<]
        [Op::DecPtr(n)] => Some(vec![Op::OptSearchZeroLeft(*n)]),

        _ => None,
    }
}

fn exec<R: io::Read, W: io::Write>(ops: &Vec<Op>, input: &mut R, output: &mut W) {
    let mut mem = [0 as u8; 32765];
    let mut ptr: usize = mem.len() / 2 + 1;

    let mut i = 0;
    while i < ops.len() {
        match ops[i] {
            Op::IncPtr(n) => ptr += n,
            Op::DecPtr(n) => ptr -= n,
            Op::IncVal(n) => mem[ptr] = mem[ptr].wrapping_add(n),
            Op::DecVal(n) => mem[ptr] = mem[ptr].wrapping_sub(n),
            Op::WriteVal => match output.write(&[mem[ptr]]) {
                Err(err) => panic!(err),
                _ => (),
            },
            Op::ReadVal => {
                let mut buf = [0; 1];
                match input.read(&mut buf) {
                    Ok(1) => mem[ptr] = buf[0],
                    Ok(_) => (),
                    Err(err) => panic!(err),
                }
            }
            Op::LoopBegin(pos) => {
                if mem[ptr] == 0 {
                    i = pos;
                }
            }
            Op::LoopEnd(pos) => i = pos - 1,
            Op::OptSetValZero => mem[ptr] = 0,
            Op::OptAddValRight(n) => {
                mem[ptr + n] = mem[ptr + n].wrapping_add(mem[ptr]);
                mem[ptr] = 0;
            }
            Op::OptAddValLeft(n) => {
                mem[ptr - n] = mem[ptr - n].wrapping_add(mem[ptr]);
                mem[ptr] = 0;
            }
            Op::OptSubValRight(n) => {
                mem[ptr + n] = mem[ptr + n].wrapping_sub(mem[ptr]);
                mem[ptr] = 0;
            }
            Op::OptSubValLeft(n) => {
                mem[ptr - n] = mem[ptr - n].wrapping_sub(mem[ptr]);
                mem[ptr] = 0;
            }
            Op::OptSearchZeroRight(n) => {
                while mem[ptr] != 0 {
                    ptr += n;
                }
            }
            Op::OptSearchZeroLeft(n) => {
                while mem[ptr] != 0 {
                    ptr -= n;
                }
            }
        }
        i += 1;
    }
    if let Err(err) = output.flush() {
        panic!(err);
    }
}
