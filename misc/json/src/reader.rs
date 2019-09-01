pub trait Reader {
    fn cur(&self) -> Option<u8>;
    fn slice(&self, n: usize) -> Option<&[u8]>;
    fn mov(&mut self, n: usize);
}

pub struct BytesReader<'a> {
    index: usize,
    bytes: &'a [u8],
    len: usize,
}

impl<'a> BytesReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        BytesReader {
            index: 0,
            bytes: bytes,
            len: bytes.len(),
        }
    }
}

impl<'a> Reader for BytesReader<'a> {
    fn cur(&self) -> Option<u8> {
        if self.index < self.len {
            Some(self.bytes[self.index])
        } else {
            None
        }
    }
    fn slice(&self, n: usize) -> Option<&[u8]> {
        let pos = self.index + n;
        if pos <= self.len {
            Some(&self.bytes[self.index..pos])
        } else {
            None
        }
    }
    fn mov(&mut self, n: usize) {
        self.index = self.index + n;
    }
}
