use brainfuck::eval_jit;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!(format!("Usage: {} <filename>", &args[0]));
    }
    eval_jit(
        &fs::read(&args[1]).unwrap(),
        &mut io::stdin().lock(),
        &mut io::stdout().lock(),
    );
}
