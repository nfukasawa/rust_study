use super::operations::Op;
use std::io;

pub struct Interpreter<R: io::Read, W: io::Write> {
    mem: Vec<u8>,
    ptr: usize,
    input: R,
    output: W,
}

impl<R: io::Read, W: io::Write> Interpreter<R, W> {
    pub fn new(input: R, output: W) -> Self {
        let size = 65535;
        Interpreter {
            mem: vec![0 as u8; size],
            ptr: size / 2 + 1,
            input,
            output,
        }
    }

    pub fn interpret(&mut self, ops: &[Op]) {
        let mut pc = 0;
        while pc < ops.len() {
            match &ops[pc] {
                Op::MovPtr(n) => self.ptr = offset_ptr(self.ptr, *n),
                Op::AddVal(offset, v) => {
                    let p = offset_ptr(self.ptr, *offset);
                    self.mem[p] = add_val_wrap(self.mem[p], *v);
                }
                Op::WriteVal(offset) => {
                    if let Err(err) = self
                        .output
                        .write(&[self.mem[offset_ptr(self.ptr, *offset)]])
                    {
                        panic!(err);
                    }
                }
                Op::ReadVal(offset) => {
                    let mut buf = [0; 1];
                    match self.input.read(&mut buf) {
                        Ok(1) => self.mem[offset_ptr(self.ptr, *offset)] = buf[0],
                        Err(err) => panic!(err),
                        _ => panic!("read failed"),
                    }
                }
                Op::LoopBegin(p) => {
                    if self.mem[self.ptr] == 0 {
                        pc = *p;
                    }
                }
                Op::LoopEnd(p) => pc = *p - 1,
                Op::Loop(inner) => {
                    while self.mem[self.ptr] != 0 {
                        self.interpret(inner);
                    }
                }
                Op::ClearVal(offset) => self.mem[offset_ptr(self.ptr, *offset)] = 0,
                Op::MoveMulVal(offset, n, mul) => {
                    let p = offset_ptr(self.ptr, *offset);
                    let to = offset_ptr(p, *n);
                    self.mem[to] = add_val_wrap(self.mem[to], mul_val(self.mem[p], *mul));
                    self.mem[p] = 0;
                }
                Op::MoveMulValN(offset, params) => {
                    let p = offset_ptr(self.ptr, *offset);
                    for (n, mul) in params.iter() {
                        let to = offset_ptr(p, *n);
                        self.mem[to] = add_val_wrap(self.mem[to], mul_val(self.mem[p], *mul));
                    }
                    self.mem[p] = 0;
                }
                Op::SkipToZero(n) => {
                    while self.mem[self.ptr] != 0 {
                        self.ptr = offset_ptr(self.ptr, *n);
                    }
                }
            }
            pc += 1;
        }

        if let Err(err) = self.output.flush() {
            panic!(err);
        }
    }
}

#[inline]
fn add_val_wrap(v: u8, d: i16) -> u8 {
    i16::from(v).wrapping_add(d) as u8
}

#[inline]
fn mul_val(v: u8, m: i16) -> i16 {
    i16::from(v) * m
}

#[inline]
fn offset_ptr(ptr: usize, d: isize) -> usize {
    (ptr as isize + d) as usize
}
