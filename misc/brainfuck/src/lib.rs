use std::io;

mod interpreter;
mod jit;
mod operations;
mod parser;

use interpreter::Interpreter;
use jit::JIT;
use parser::Parser;

pub fn eval<R: io::Read, W: io::Write>(code: &[u8], input: R, output: W) {
    let ops = Parser::new().parse(code);
    Interpreter::new(input, output).exec(&ops);
}

pub fn eval_jit<R: io::Read, W: io::Write>(code: &[u8], input: R, output: W) {
    let ops = Parser::new().parse(code);
    JIT::new(input, output).exec(&ops);
}
