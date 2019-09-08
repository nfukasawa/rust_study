use std::io;

mod interpreter;
mod operations;
mod parser;

use interpreter::Interpreter;
use parser::Parser;

pub fn eval<R: io::Read, W: io::Write>(code: &[u8], input: R, output: W) {
    let ops = Parser::new().parse(code);
    Interpreter::new(input, output).interpret(&ops);
}
