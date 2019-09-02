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

#[derive(Clone, Debug, PartialEq)]
enum Op {
    MovPtr(isize),
    AddVal(i16),
    WriteVal,
    ReadVal,
    LoopBegin(usize),
    LoopEnd(usize),

    ClearVal,
    MoveMulVal(isize, i16),
    MoveMulValN(Vec<(isize, i16)>),
    SkipToZero(isize),
}

fn opertions(code: &[u8]) -> Vec<Op> {
    let l = code.len();
    let mut ops = Vec::with_capacity(l);
    let mut loop_stack = Vec::new();

    let mut pos = 0;
    while pos < l {
        match code[pos] {
            b'>' | b'<' => {
                let mut v = 0;
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
                ops.push(Op::MovPtr(v));
            }
            b'+' | b'-' => {
                let mut v: i16 = 0;
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
                ops.push(Op::AddVal(v));
            }
            b'.' => ops.push(Op::WriteVal),
            b',' => ops.push(Op::ReadVal),
            b'[' => {
                loop_stack.push(ops.len());
                ops.push(Op::LoopBegin(std::usize::MAX));
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
        [] => return None,

        // [-] [+]
        [Op::AddVal(1)] | [Op::AddVal(-1)] => Some(vec![Op::ClearVal]),

        // [>>+<<-] [<<+>>-] [>>-<<-] [<<->>-] [>>+<<+] [<<+>>+] [>>-<<+] [<<->>+]
        // [->>+<<] [-<<+>>] [->>-<<] [-<<->>] [+>>+<<] [+<<+>>] [+>>-<<] [+<<->>]
        [Op::MovPtr(n), Op::AddVal(mul), Op::MovPtr(m), Op::AddVal(s)]
        | [Op::AddVal(s), Op::MovPtr(n), Op::AddVal(mul), Op::MovPtr(m)]
            if *n == -*m && s.abs() == 1 =>
        {
            Some(vec![Op::MoveMulVal(*n, -*s * *mul)])
        }

        // [>>] [<<]
        [Op::MovPtr(n)] => Some(vec![Op::SkipToZero(*n)]),

        _ => optimize_multi_move(ops),
    }
}

// like [->+++>+++++++<<]
fn optimize_multi_move(ops: &[Op]) -> Option<Vec<Op>> {
    let ops0: &[Op];
    let s: i16;
    if let Some(Op::AddVal(n)) = ops.first() {
        s = *n;
        ops0 = &ops[1..];
    } else if let Some(Op::AddVal(n)) = ops.last() {
        s = *n;
        ops0 = &ops[..ops.len() - 1];
    } else {
        return None;
    }

    if s.abs() != 1 {
        return None;
    }

    let offset: isize;
    if let Some(Op::MovPtr(n)) = ops0.last() {
        offset = *n;
    } else {
        return None;
    }

    let ops1 = &ops0[..ops0.len() - 1];

    let mut moves = Vec::new();
    let mut pos = 0;
    for chunk in ops1.chunks(2) {
        match chunk {
            [Op::MovPtr(n), Op::AddVal(mul)] => {
                pos += *n;
                moves.push((pos, -s * *mul));
            }
            _ => return None,
        }
    }

    if offset + pos == 0 {
        Some(vec![Op::MoveMulValN(moves)])
    } else {
        None
    }
}

fn exec<R: io::Read, W: io::Write>(ops: &Vec<Op>, input: &mut R, output: &mut W) {
    let mut mem = [0 as u8; 32765];
    let mut ptr: usize = mem.len() / 2 + 1;

    let mut pc = 0;
    while pc < ops.len() {
        match &ops[pc] {
            Op::MovPtr(n) => ptr = (ptr as isize + *n) as usize,
            Op::AddVal(n) => mem[ptr] = (mem[ptr] as i16).wrapping_add(*n) as u8,
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
                    pc = *pos;
                }
            }
            Op::LoopEnd(pos) => pc = *pos - 1,

            Op::ClearVal => mem[ptr] = 0,
            Op::MoveMulVal(n, x) => {
                let to = (ptr as isize + *n) as usize;
                mem[to] = ((mem[to] as i16).wrapping_add(mem[ptr] as i16 * *x)) as u8;
                mem[ptr] = 0;
            }
            Op::MoveMulValN(ps) => {
                for p in ps.iter() {
                    let offset = p.0;
                    let mul = p.1;
                    let to = (ptr as isize + offset) as usize;
                    mem[to] = ((mem[to] as i16).wrapping_add(mem[ptr] as i16 * mul)) as u8;
                }
                mem[ptr] = 0;
            }
            Op::SkipToZero(n) => {
                while mem[ptr] != 0 {
                    ptr = (ptr as isize + *n) as usize;
                }
            }
        }
        pc += 1;
    }
    if let Err(err) = output.flush() {
        panic!(err);
    }
}
