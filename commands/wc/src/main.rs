extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read, Result};

fn main() {
    let (opts, files) = Opts::parse();
    if let Some(files) = files {
        for filename in files {
            // TODO: expand wildcard
            match File::open(filename.as_str()) {
                Ok(file) => {
                    count(file, &opts).print(&filename, &opts);
                }
                Err(err) => eprintln!("{}", err),
            }
        }
    } else {
        count(&mut stdin().lock(), &opts).print(&"".to_string(), &opts);
    }
}

#[derive(Debug, Clone)]
struct Opts {
    lines: bool,
    words: bool,
    chars: bool,
    bytes: bool,
}

impl Opts {
    fn parse() -> (Self, Option<Vec<String>>) {
        let app = App::new("wc")
            .arg(Arg::with_name("files").min_values(0))
            .arg(Arg::with_name("lines").short("l").long("lines"))
            .arg(Arg::with_name("words").short("w").long("words"))
            .arg(Arg::with_name("chars").short("m").long("chart"))
            .arg(Arg::with_name("bytes").short("c").long("bytes"));
        // TODO max-line-length

        let m: ArgMatches = app.get_matches();

        let mut lines = m.is_present("lines");
        let mut words = m.is_present("wods");
        let mut bytes = m.is_present("bytes");
        let chars = m.is_present("chars");
        if !bytes && !chars && !lines && !words {
            // default
            bytes = true;
            words = true;
            lines = true;
        }

        let files = match m.values_of("files") {
            Some(files) => Some(files.map(|f| f.to_string()).collect()),
            None => None,
        };

        (
            Self {
                bytes,
                chars,
                words,
                lines,
            },
            files,
        )
    }

    fn is_bytes_only(&self) -> bool {
        return self.bytes && !(self.lines || self.words || self.chars);
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Counts {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
}

impl Counts {
    fn print(&self, filename: &String, opts: &Opts) {
        let mut vals = Vec::new();
        if opts.lines {
            vals.push(self.lines);
        }
        if opts.words {
            vals.push(self.words);
        }
        if opts.chars {
            vals.push(self.chars);
        }
        if opts.bytes {
            vals.push(self.bytes)
        }

        for n in vals.iter() {
            if *n > 0 {
                print!("{: >8} ", n) // TODO: calc width
            }
        }
        if filename != "" {
            print!("{}", filename);
        }
        print!("\n");
    }
}

fn count<R: Read>(input: R, opts: &Opts) -> Counts {
    if opts.is_bytes_only() {
        let mut input = ByteCountReader::new(input);
        loop {
            let mut buf = vec![0; 8192];
            match input.read(&mut buf) {
                Ok(0) => break,
                Err(err) => panic!(err),
                _ => (),
            }
        }
        return Counts {
            bytes: input.bytes,
            ..Default::default()
        };
    }

    if opts.bytes {
        let mut input = ByteCountReader::new(input);
        let mut c = count_partial(&mut BufReader::new(&mut input), opts);
        c.bytes = input.bytes;
        c
    } else {
        count_partial(&mut BufReader::new(input), opts)
    }
}

fn count_partial<R: BufRead>(input: &mut R, opts: &Opts) -> Counts {
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    let mut line = String::new();
    loop {
        line.clear();
        match input.read_line(&mut line) {
            Ok(0) => break,
            Ok(n) => {
                lines += 1;
                chars += n;
                if !opts.words {
                    continue;
                }
                for word in line.split_whitespace() {
                    if !word.is_empty() {
                        words += 1;
                    }
                }
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
    Counts {
        lines: if opts.lines { lines } else { 0 },
        words: if opts.words { words } else { 0 },
        chars: if opts.chars { chars } else { 0 },
        ..Default::default()
    }
}

struct ByteCountReader<R: Read> {
    input: R,
    bytes: usize,
}

impl<R: Read> ByteCountReader<R> {
    fn new(input: R) -> Self {
        ByteCountReader { input, bytes: 0 }
    }
}

impl<R: Read> Read for ByteCountReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.input.read(buf);
        if let Ok(n) = result {
            self.bytes += n;
        }
        result
    }
}
