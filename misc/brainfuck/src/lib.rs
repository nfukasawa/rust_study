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
        self.exec(&ops);
    }

    fn exec(&mut self, ops: &Vec<Op>) {
        let mut mem = [0 as u8; 32765];
        let mut ptr: usize = mem.len() / 2 + 1;

        let mut i = 0;
        while i < ops.len() {
            println!("OP: {:?}", ops[i]);
            match ops[i] {
                Op::IncPtr(n) => ptr += n,
                Op::DecPtr(n) => ptr -= n,
                Op::IncVal(n) => mem[ptr] += n,
                Op::DecVal(n) => mem[ptr] -= n,
                Op::WriteVal => match self.output.write(&[mem[ptr]]) {
                    Err(err) => panic!(err),
                    _ => (),
                },
                Op::ReadVal => {
                    let mut buf = [0; 1];
                    match self.input.read(&mut buf) {
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
            }
            i += 1;
        }
        if let Err(err) = self.output.flush() {
            panic!(err);
        }
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
                    ops[i] = Op::LoopBegin(ops.len());
                    ops.push(Op::LoopEnd(i));
                } else {
                    panic!("corresponding '[' not found: {}", pos);
                }
            }
            b' ' | b'\n' | b'\r' | b'\t' => (),
            _ => panic!("invalid operation: {}", pos),
        };
        pos += 1;
    }
    ops
}
