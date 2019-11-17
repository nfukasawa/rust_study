use peg::parser;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::{Child, Command, Stdio};

fn main() -> io::Result<()> {
    let mut reader = Reader::new();
    loop {
        match reader.commands() {
            Some(cmds) => cmds.exec()?,
            None => (),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Commands {
    cmds: Vec<Cmd>,
}

impl Commands {
    pub fn new(cmds: Vec<Cmd>) -> Self {
        Commands { cmds }
    }

    pub fn exec(&self) -> io::Result<()> {
        let mut children = Vec::new();
        for cmd in &self.cmds {
            match cmd.process() {
                Ok(child) => children.push(child),
                Err(err) => {
                    let stderr = io::stderr();
                    writeln!(stderr.lock(), "{}", err)?;
                }
            }
        }

        let mut i = 1;
        while i < children.len() {
            let (c1, c2) = children.split_at_mut(i);
            let pipe_out = c1[i - 1].stdout.as_mut().unwrap();
            let pipe_in = c2[0].stdin.as_mut().unwrap();
            io::copy(pipe_out, pipe_in)?;
            i += 1;
        }

        for mut child in children {
            child.wait()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Cmd {
    cmd: String,
    opts: Vec<String>,
    input: IO,
    output: IO,
}

impl Cmd {
    pub fn new<S: Into<String>>(cmd: S) -> Self {
        Self {
            cmd: cmd.into(),
            opts: Vec::new(),
            input: IO::Std,
            output: IO::Std,
        }
    }

    pub fn process(&self) -> Result<Child, io::Error> {
        let stdin = self.input.stdin()?;
        let stdout = self.output.stdout()?;
        Ok(Command::new(&self.cmd)
            .args(&self.opts)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()?)
    }

    pub fn add_opt<S: Into<String>>(&mut self, opt: S) {
        self.opts.push(opt.into());
    }
    pub fn set_input(&mut self, input: IO) {
        self.input = input;
    }
    pub fn set_output(&mut self, output: IO) {
        self.output = output;
    }
}

#[derive(Debug, Clone)]
pub enum IO {
    Std,
    Pipe,
    Redirect(String),
}

impl IO {
    pub fn redirect<S: Into<String>>(file: S) -> IO {
        IO::Redirect(file.into())
    }

    pub fn stdin(&self) -> Result<Stdio, io::Error> {
        match self {
            IO::Std => Ok(Stdio::inherit()),
            IO::Pipe => Ok(Stdio::piped()),
            IO::Redirect(file) => Ok(Stdio::from(File::open(file)?)),
        }
    }

    pub fn stdout(&self) -> Result<Stdio, io::Error> {
        match self {
            IO::Std => Ok(Stdio::inherit()),
            IO::Pipe => Ok(Stdio::piped()),
            IO::Redirect(file) => Ok(Stdio::from(File::create(file)?)),
        }
    }
}

struct Reader {}
impl Reader {
    pub fn new() -> Self {
        Reader {}
    }

    pub fn commands(&mut self) -> Option<Commands> {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();

        let mut line = String::new();
        self.prompt().expect("io error");

        // TODO: multi line
        if let Some(cmds) = match stdin.read_line(&mut line) {
            Ok(_) => self.parse_line(&line),
            Err(_) => None,
        } {
            Some(Commands::new(cmds))
        } else {
            None
        }
    }

    fn parse_line(&self, line: &String) -> Option<Vec<Cmd>> {
        match shell::commands(line) {
            Ok(cmds) => Some(cmds),
            Err(_) => None,
        }
    }

    fn prompt(&self) -> io::Result<()> {
        write!(io::stderr().lock(), "> ")
    }
}

#[derive(Debug, Clone)]
enum OptType {
    Opt(String),
    RedirectIn(String),
    RedirectOut(String),
}

fn build_commands(cmds: &Vec<(String, Vec<OptType>)>) -> Vec<Cmd> {
    let mut ret = Vec::new();
    let mut i = 0;
    let n = cmds.len();
    for (cmd, opts) in cmds {
        let mut c = Cmd::new(cmd);
        for opt in opts {
            match opt {
                OptType::Opt(opt) => c.add_opt(opt),
                OptType::RedirectIn(file) => c.set_input(IO::redirect(file)),
                OptType::RedirectOut(file) => c.set_output(IO::redirect(file)),
            }
        }
        if i != 0 {
            c.set_input(IO::Pipe);
        }
        if i != n - 1 {
            c.set_output(IO::Pipe);
        }
        ret.push(c);
        i += 1;
    }
    ret
}

// TODO: quoted, etc...
parser! {
    grammar shell() for str {
        pub rule commands() -> Vec<Cmd>
          = cmds:command() ** ("|") { build_commands(&cmds) }

        rule command() -> (String, Vec<OptType>)
          = _ cmd:token() opts:(option() / redirect_out() / redirect_in())* _ { (cmd, opts)}

        rule option() -> OptType
          = _ opt:token() _ { OptType::Opt(opt) }

        rule redirect_in() -> OptType
          = _ "<" _ file:token() _ { OptType::RedirectIn(file) }

        rule redirect_out() -> OptType
          = _ ">" _ file:token() _ { OptType::RedirectOut(file) }

        rule token() -> String
            = t:$((!['<' | '>' | '|' | ' ' | '"' | '\'' | '\t' | '\n'] [_])+) { t.into() }

        rule _() = [' ' | '\t' | '\n']*
    }
}
