use std::char;
use std::collections::HashMap;

use super::error::Err;
use super::reader::Reader;
use super::value::Value;

pub struct Parser<R: Reader> {
    reader: R,
}

impl<R: Reader> Parser<R> {
    pub fn new(r: R) -> Self {
        Parser { reader: r }
    }

    pub fn parse(&mut self) -> Result<Value, Err> {
        let val = self.parse_value()?;
        if self.skip_spaces().is_none() {
            Ok(val)
        } else {
            Err(Err::new())
        }
    }

    fn parse_value(&mut self) -> Result<Value, Err> {
        match self.skip_spaces() {
            Some(b'"') => self.parse_string(),
            c @ Some(b'0'..=b'9') | c @ Some(b'-') => self.parse_number(c.unwrap()),
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b't') => self.parse_true(),
            Some(b'f') => self.parse_false(),
            Some(b'n') => self.parse_null(),
            _ => return Err(Err::new()),
        }
    }

    fn parse_string(&mut self) -> Result<Value, Err> {
        let mut s = String::new();
        let mut buf = Vec::new();
        let mut closed = false;
        let mut escaped = false;

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
                escaped = false;
            } else {
                match b {
                    b'\\' => {
                        s.push_str(bytes_to_string(buf.as_slice()).as_str());
                        buf.clear();
                        escaped = true;
                    }
                    b'"' => {
                        s.push_str(bytes_to_string(buf.as_slice()).as_str());
                        buf.clear();
                        closed = true;
                        break;
                    }
                    _ => {
                        buf.push(b);
                    }
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
        match self.slice(4) {
            Some(bs) => {
                let mut n: u32 = 0;
                for b in bs {
                    n *= 16;
                    n += match b {
                        b'0'..=b'9' => b - b'0',
                        b'A'..=b'F' => b - b'A' + 10,
                        b'a'..=b'f' => b - b'a' + 10,
                        _ => return Err(Err::new()),
                    } as u32;
                }
                self.mov(4);
                match char::from_u32(n) {
                    Some(c) => Ok(c),
                    None => Err(Err::new()),
                }
            }
            None => Err(Err::new()),
        }
    }

    fn parse_number(&mut self, first: u8) -> Result<Value, Err> {
        enum State {
            Minus,
            Zero,
            Dot,
            Integer,
            Fraction,
            ExpSign,
            Exp,
        }

        let mut state = match first {
            b'-' => State::Minus,
            b'0' => State::Zero,
            b'1'..=b'9' => State::Integer,
            _ => panic!("invalid first byte."),
        };

        let mut s = String::new();
        s.push(first as char);
        while let Some(b) = self.cur() {
            state = match state {
                State::Minus => match b {
                    b'0' => State::Zero,
                    b'1'..=b'9' => State::Integer,
                    _ => break,
                },
                State::Zero => match b {
                    b'.' => State::Dot,
                    _ => break,
                },
                State::Dot => match b {
                    b'0'..=b'9' => State::Fraction,
                    _ => break,
                },
                State::Integer => match b {
                    b'0'..=b'9' => State::Integer,
                    b'.' => State::Fraction,
                    b'e' | b'E' => State::ExpSign,
                    _ => break,
                },
                State::Fraction => match b {
                    b'0'..=b'9' => State::Integer,
                    b'e' | b'E' => State::ExpSign,
                    _ => break,
                },
                State::ExpSign => match b {
                    b'+' | b'-' => State::Exp,
                    _ => break,
                },
                State::Exp => match b {
                    b'0'..=b'9' => State::Exp,
                    _ => break,
                },
            };
            s.push(b as char);
            self.forward();
        }

        Ok(Value::Number(s.parse().unwrap()))
    }

    fn parse_object(&mut self) -> Result<Value, Err> {
        let mut obj = HashMap::new();
        loop {
            let key;
            match self.skip_spaces() {
                Some(b'"') => {
                    key = self.parse_string()?.as_string().unwrap().clone();
                }
                _ => return Err(Err::new()),
            }

            match self.skip_spaces() {
                Some(b':') => (),
                _ => return Err(Err::new()),
            }

            obj.insert(key, self.parse_value()?);

            match self.skip_spaces() {
                Some(b',') => (),
                Some(b'}') => break,
                _ => return Err(Err::new()),
            }
        }
        Ok(Value::Object(Box::new(obj)))
    }

    fn parse_array(&mut self) -> Result<Value, Err> {
        let mut arr = Vec::new();
        loop {
            let v = self.parse_value()?;
            arr.push(v);
            match self.skip_spaces() {
                Some(b',') => (),
                Some(b']') => break,
                _ => return Err(Err::new()),
            }
        }
        Ok(Value::Array(arr))
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

    fn match_next_bytes(&mut self, bs: &[u8]) -> Result<(), Err> {
        match self.slice(bs.len()) {
            Some(s) => {
                if bs.eq(s) {
                    self.mov(bs.len());
                    Ok(())
                } else {
                    Err(Err::new())
                }
            }
            None => Err(Err::new()),
        }
    }

    fn next(&mut self) -> Option<u8> {
        let b = self.reader.cur();
        if b.is_some() {
            self.reader.mov(1);
            b
        } else {
            None
        }
    }

    fn cur(&self) -> Option<u8> {
        self.reader.cur()
    }

    fn slice(&self, n: usize) -> Option<&[u8]> {
        self.reader.slice(n)
    }

    fn mov(&mut self, n: usize) {
        self.reader.mov(n)
    }

    fn forward(&mut self) {
        self.reader.mov(1);
    }
}

fn bytes_to_string(bs: &[u8]) -> String {
    String::from_utf8(bs.to_vec()).unwrap()
}
