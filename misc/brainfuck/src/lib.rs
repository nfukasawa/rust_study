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
    AddVal(isize, i16),
    WriteVal(isize),
    ReadVal(isize),
    LoopBegin(usize),
    LoopEnd(usize),

    ClearVal(isize),
    MoveMulVal(isize, isize, i16),
    MoveMulValN(isize, Vec<(isize, i16)>),
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

    optimize_offsets(ops.as_ref())
}

fn optimize_loop(ops: &[Op]) -> Option<Vec<Op>> {
    match ops {
        [] => return None,

        // [-] [+]
        [Op::AddVal(_, 1)] | [Op::AddVal(_, -1)] => Some(vec![Op::ClearVal(0)]),

        // [>>+<<-] [<<+>>-] [>>-<<-] [<<->>-] [>>+<<+] [<<+>>+] [>>-<<+] [<<->>+]
        // [->>+<<] [-<<+>>] [->>-<<] [-<<->>] [+>>+<<] [+<<+>>] [+>>-<<] [+<<->>]
        [Op::MovPtr(n), Op::AddVal(_, mul), Op::MovPtr(m), Op::AddVal(_, s)]
        | [Op::AddVal(_, s), Op::MovPtr(n), Op::AddVal(_, mul), Op::MovPtr(m)]
            if *n == -*m && s.abs() == 1 =>
        {
            Some(vec![Op::MoveMulVal(0, *n, -*s * *mul)])
        }

        // [>>] [<<]
        [Op::MovPtr(n)] => Some(vec![Op::SkipToZero(*n)]),

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
            }
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
    optimized
}

fn exec<R: io::Read, W: io::Write>(ops: &Vec<Op>, input: &mut R, output: &mut W) {
    let mut mem = [0 as u8; 65535];
    let mut ptr: usize = mem.len() / 2 + 1;

    let mut pc = 0;
    while pc < ops.len() {
        match &ops[pc] {
            Op::MovPtr(n) => ptr = offset_ptr(ptr, *n),
            Op::AddVal(offset, v) => {
                let p = offset_ptr(ptr, *offset);
                mem[p] = add_val_wrap(mem[p], *v);
            }
            Op::WriteVal(offset) => match output.write(&[mem[offset_ptr(ptr, *offset)]]) {
                Err(err) => panic!(err),
                _ => (),
            },
            Op::ReadVal(offset) => {
                let mut buf = [0; 1];
                match input.read(&mut buf) {
                    Ok(1) => mem[offset_ptr(ptr, *offset)] = buf[0],
                    Err(err) => panic!(err),
                    _ => panic!("read failed"),
                }
            }
            Op::LoopBegin(p) => {
                if mem[ptr] == 0 {
                    pc = *p;
                }
            }
            Op::LoopEnd(p) => pc = *p - 1,
            Op::ClearVal(offset) => mem[offset_ptr(ptr, *offset)] = 0,
            Op::MoveMulVal(offset, n, mul) => {
                let p = offset_ptr(ptr, *offset);
                let to = offset_ptr(p, *n);
                mem[to] = add_val_wrap(mem[to], mul_val(mem[p], *mul));
                mem[p] = 0;
            }
            Op::MoveMulValN(offset, params) => {
                let p = offset_ptr(ptr, *offset);
                for (n, mul) in params.iter() {
                    let to = offset_ptr(p, *n);
                    mem[to] = add_val_wrap(mem[to], mul_val(mem[p], *mul));
                }
                mem[p] = 0;
            }
            Op::SkipToZero(n) => {
                while mem[ptr] != 0 {
                    ptr = offset_ptr(ptr, *n);
                }
            }
        }
        pc += 1;
    }
    if let Err(err) = output.flush() {
        panic!(err);
    }
}

#[inline]
fn add_val_wrap(v: u8, d: i16) -> u8 {
    (v as i16).wrapping_add(d) as u8
}

#[inline]
fn mul_val(v: u8, m: i16) -> i16 {
    v as i16 * m
}

#[inline]
fn offset_ptr(ptr: usize, d: isize) -> usize {
    (ptr as isize + d) as usize
}

#[allow(dead_code)]
impl Op {
    fn ch(&self) -> char {
        match self {
            Op::MovPtr(x) => {
                if *x > 0 {
                    '>'
                } else {
                    '<'
                }
            }
            Op::AddVal(_, x) => {
                if *x > 0 {
                    '+'
                } else {
                    '-'
                }
            }
            Op::WriteVal(_) => '.',
            Op::ReadVal(_) => ',',
            Op::LoopBegin(_) => '[',
            Op::LoopEnd(_) => ']',
            Op::ClearVal(_) => 'c',
            Op::MoveMulVal(_, _, _) => 'm',
            Op::MoveMulValN(_, _) => 'M',
            Op::SkipToZero(_) => 's',
        }
    }
}
#[allow(dead_code)]
fn to_str(ops: &[Op]) -> String {
    ops.iter().map(|op| op.ch()).collect()
}
