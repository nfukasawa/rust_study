use std::char;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Number(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Err {} // TODO

impl Err {
    // TODO: embed error info: cause, line, position, ...
    fn new() -> Self {
        Err {}
    }
}

impl FromStr for Value {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(s.as_bytes()).parse()
    }
}

struct Parser<'a> {
    index: usize,
    bytes: &'a [u8],
    len: usize,
}

impl<'a> Parser<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Parser {
            index: 0,
            bytes: bytes,
            len: bytes.len(),
        }
    }

    fn parse(&mut self) -> Result<Value, Err> {
        let val = self.parse_value()?;
        if self.skip_spaces().is_none() {
            Ok(val)
        } else {
            Err(Err::new())
        }
    }

    fn parse_value(&mut self) -> Result<Value, Err> {
        if let Some(b) = self.skip_spaces() {
            match b {
                b'"' => self.parse_string(),
                b'0'...b'9' | b'-' => self.parse_number(),
                b'{' => self.parse_object(),
                b'[' => self.parse_array(),
                b't' => self.parse_true(),
                b'f' => self.parse_false(),
                b'n' => self.parse_null(),
                _ => return Err(Err::new()),
            }
        } else {
            Err(Err::new())
        }
    }

    fn parse_string(&mut self) -> Result<Value, Err> {
        let mut escaped = false;
        let mut s = String::new();
        let mut head = self.pos();
        let mut closed = false;

        while let Some(b) = self.next() {
            if escaped {
                s.push(match b {
                    b'"' | b'\\' | b'/' => b as char,
                    b'b' => '\x08',
                    b'f' => '\x0C',
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'u' => self.hex_to_char()?,
                    _ => return Err(Err::new()),
                });
                head = self.index;
                escaped = false;
            } else {
                match b {
                    b'\\' => {
                        s.push_str(self.substr_force(head, self.pos() - 1).as_str());
                        escaped = true;
                    }
                    b'"' => {
                        s.push_str(self.substr_force(head, self.pos() - 1).as_str());
                        closed = true;
                        break;
                    }
                    _ => (),
                }
            }
        }

        if closed {
            Ok(Value::String(s))
        } else {
            Err(Err::new())
        }
    }

    fn hex_to_char(&mut self) -> Result<char, Err> {
        match self.next_bytes(4) {
            None => Err(Err::new()),
            Some(bs) => {
                let mut n: u32 = 0;
                for b in bs {
                    n *= 16;
                    n += match b {
                        b'0'...b'9' => b - b'0',
                        b'A'...b'F' => b - b'A' + 10,
                        b'a'...b'f' => b - b'a' + 10,
                        _ => return Err(Err::new()),
                    } as u32;
                }
                match char::from_u32(n) {
                    Some(c) => Ok(c),
                    None => Err(Err::new()),
                }
            }
        }
    }

    fn substr_force(&mut self, from: usize, to: usize) -> String {
        bytes_to_string(self.slice_bytes(from, to).unwrap())
    }

    fn parse_number(&mut self) -> Result<Value, Err> {
        self.back();
        let head = self.pos();

        enum State {
            Init,
            Minus,
            Integer,
            Fraction,
            ExpSign,
            Exp,
        }
        let mut state = State::Init;

        while let Some(b) = self.next() {
            state = match state {
                State::Init => match b {
                    b'-' => State::Minus,
                    b'0' => {
                        self.match_next_bytes(&[b'.'])?;
                        State::Fraction
                    }
                    b'1'...b'9' => State::Integer,
                    _ => {
                        self.back();
                        break;
                    }
                },
                State::Minus => match b {
                    b'0' => {
                        self.match_next_bytes(&[b'.'])?;
                        State::Fraction
                    }
                    b'1'...b'9' => State::Integer,
                    _ => {
                        self.back();
                        break;
                    }
                },
                State::Integer => match b {
                    b'0'...b'9' => State::Integer,
                    b'.' => State::Fraction,
                    b'e' | b'E' => State::ExpSign,
                    _ => {
                        self.back();
                        break;
                    }
                },
                State::Fraction => match b {
                    b'0'...b'9' => State::Integer,
                    b'e' | b'E' => State::ExpSign,
                    _ => {
                        self.back();
                        break;
                    }
                },
                State::ExpSign => match b {
                    b'+' | b'-' => State::Exp,
                    _ => {
                        self.back();
                        break;
                    }
                },
                State::Exp => match b {
                    b'0'...b'9' => State::Exp,
                    _ => {
                        self.back();
                        break;
                    }
                },
            }
        }

        let mut s = String::new();
        for b in self.slice_bytes(head, self.pos()).unwrap() {
            s.push(*b as char);
        }
        Ok(Value::Number(s.parse().unwrap()))
    }

    fn parse_object(&mut self) -> Result<Value, Err> {
        // TODO
        Ok(Value::Boolean(true))
    }

    fn parse_array(&mut self) -> Result<Value, Err> {
        // TODO
        Ok(Value::Boolean(true))
    }

    fn parse_true(&mut self) -> Result<Value, Err> {
        self.match_next_bytes(b"rue")?;
        Ok(Value::Boolean(true))
    }

    fn parse_false(&mut self) -> Result<Value, Err> {
        self.match_next_bytes(b"alse")?;
        Ok(Value::Boolean(false))
    }

    fn parse_null(&mut self) -> Result<Value, Err> {
        self.match_next_bytes(b"ull")?;
        Ok(Value::Null)
    }

    fn skip_spaces(&mut self) -> Option<u8> {
        while let Some(b) = self.next() {
            match b {
                b' ' | b'\n' | b'\r' | b'\t' => (),
                _ => return Some(b),
            }
        }
        None
    }

    fn next(&mut self) -> Option<u8> {
        if self.index < self.len {
            let ret = Some(self.bytes[self.index]);
            self.index += 1;
            ret
        } else {
            None
        }
    }

    fn back(&mut self) {
        self.index -= 1;
    }

    fn pos(&self) -> usize {
        self.index
    }

    fn slice_bytes(&self, from: usize, to: usize) -> Option<&[u8]> {
        if from > to || to > self.len {
            None
        } else {
            Some(&self.bytes[from..to])
        }
    }

    fn next_bytes(&mut self, n: usize) -> Option<&[u8]> {
        if self.index + n - 1 < self.len {
            let head = self.index;
            self.index += n;
            Some(&self.bytes[head..self.index])
        } else {
            None
        }
    }

    fn match_next_bytes(&mut self, bs: &[u8]) -> Result<(), Err> {
        match self.next_bytes(bs.len()) {
            Some(bs0) => {
                if bs.eq(bs0) {
                    Ok(())
                } else {
                    Err(Err::new())
                }
            }
            None => Err(Err::new()),
        }
    }
}

fn bytes_to_string(bs: &[u8]) -> String {
    String::from_utf8(bs.to_vec()).unwrap()
}
