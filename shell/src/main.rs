use std::io;
use std::io::prelude::*;
use std::process::{Child, Command, Stdio};

fn main() -> io::Result<()> {
    let mut reader = Reader::new();
    while let Some(cmds) = reader.cmds() {
        cmds.exec()?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct Commands {
    cmds: Vec<Cmd>,
}

impl Commands {
    pub fn new(cmds: Vec<Cmd>) -> Self {
        Commands { cmds }
    }

    pub fn exec(&self) -> io::Result<()> {
        let mut children = self
            .cmds
            .iter()
            .map(|cmd| cmd.process())
            .collect::<Vec<Child>>();

        let mut i = 1;
        while i < children.len() {
            let (c1, c2) = children.split_at_mut(i);
            io::copy(
                c1[i - 1].stdout.as_mut().unwrap(),
                c2[0].stdin.as_mut().unwrap(),
            )
            .unwrap();
            i += 1;
        }
        for mut child in children {
            child.wait()?;
        }
        Ok(())
    }
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

    pub fn process(&self) -> Child {
        Command::new(&self.cmd)
            .args(&self.opts)
            .stdin(self.input.stdin())
            .stdout(self.output.stdout())
            .spawn()
            .expect("failed to run process") // TODO: error
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

    pub fn stdin(&self) -> Stdio {
        match self {
            IO::Pipe => Stdio::piped(),
            IO::Redirect(file) => Stdio::inherit(), // TODO
            _ => Stdio::inherit(),
        }
    }

    pub fn stdout(&self) -> Stdio {
        match self {
            IO::Pipe => Stdio::piped(),
            IO::Redirect(file) => Stdio::inherit(), // TODO
            _ => Stdio::inherit(),
        }
    }
}

struct Reader {}
impl Reader {
    pub fn new() -> Self {
        Reader {}
    }

    pub fn cmds(&mut self) -> Option<Commands> {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();

        let mut line = String::new();
        self.prompt().expect("io error");

        // TODO: multi line
        if let Some(cmds) = match stdin.read_line(&mut line) {
            Ok(_) => Some(self.parse_line(&line)),
            Err(_) => None,
        } {
            Some(Commands::new(cmds))
        } else {
            None
        }
    }

    fn parse_line(&self, line: &String) -> Vec<Cmd> {
        // TODO: use perser combinator ?
        // TODO: parse quotes, etc...
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

    fn prompt(&self) -> io::Result<()> {
        write!(io::stderr().lock(), "> ")
    }
}
