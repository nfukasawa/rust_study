use std::io;
use std::io::prelude::*;

use std::process;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let stdout = io::stdout();
    let mut stdout = io::BufWriter::new(stdout.lock());

    let stderr = io::stderr();
    let mut stderr = stderr.lock();

    prompt(&mut stderr)?;
    for line in stdin.lines() {
        let line: String = line?;

        if line.is_empty() {
            prompt(&mut stderr)?;
            continue;
        }

        let cmds = parse_line(&line);
        println!("{:?}", cmds);

        /*
        match process::Command::new(line).output() {
            Ok(res) => {
                write!(stdout, "{}", std::str::from_utf8(&res.stdout).unwrap())?;
            }
            Err(err) => {
                println!("err :{:?}", err);
            }
        }
        stdout.flush()?;
        write!(stderr, "\n")?;
        */
        prompt(&mut stderr)?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct Cmd {
    cmd: String,
    opts: Vec<String>,
    input: IO,
    output: IO,
}

impl Cmd {
    pub fn new(cmd: &str) -> Self {
        Self {
            cmd: cmd.to_string(),
            opts: Vec::new(),
            input: IO::Std,
            output: IO::Std,
        }
    }
    pub fn add_opt(&mut self, opt: &str) {
        self.opts.push(opt.to_string());
    }
    pub fn set_input(&mut self, input: IO) {
        self.input = input;
    }
    pub fn set_output(&mut self, output: IO) {
        self.output = output;
    }
}

#[derive(Debug, Clone)]
enum IO {
    Std,
    Pipe,
    Redirect(String),
}

impl IO {
    pub fn redirect(file: &str) -> IO {
        IO::Redirect(file.to_string())
    }
}

fn exec_cmds(cmds: &Vec<Cmd>) -> io::Result<()> {
    Ok(())
}

fn parse_line(line: &String) -> Vec<Cmd> {
    // TODO: use perser combinator ?
    // TODO: parse quotes, etc...
    // TODO: multi line
    let mut iter = line.split_ascii_whitespace();

    let mut cmds = Vec::new();
    let mut pipe = false;

    while let Some(token) = iter.next() {
        let mut cmd = Cmd::new(token);
        if pipe {
            cmd.set_input(IO::Pipe);
            pipe = false;
        }

        while let Some(token) = iter.next() {
            match token {
                "|" => {
                    cmd.set_output(IO::Pipe);
                    pipe = true;
                    break;
                }
                ">" => match iter.next() {
                    Some(file) => {
                        cmd.set_output(IO::redirect(file));
                        break;
                    }
                    None => {} // TODO
                },
                "<" => match iter.next() {
                    Some(file) => cmd.set_input(IO::redirect(file)),
                    None => {} // TODO
                },
                _ => cmd.add_opt(token),
            }
        }
        cmds.push(cmd);
    }
    return cmds;
}

fn prompt<W: io::Write>(out: &mut W) -> io::Result<()> {
    write!(out, "> ")
}
