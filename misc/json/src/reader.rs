pub trait Reader {
    fn pos(&self) -> usize;
    fn mov(&mut self, n: isize) -> Option<usize>;
    fn next(&mut self) -> Option<u8>;
    fn slice(&self, from: usize, to: usize) -> Option<&[u8]>;
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
    fn pos(&self) -> usize {
        self.index
    }
    fn mov(&mut self, n: isize) -> Option<usize> {
        let pos = self.index as isize + n;
        if pos < 0 || pos > self.len as isize {
            None
        } else {
            self.index = pos as usize;
            Some(self.index)
        }
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
    fn slice(&self, from: usize, to: usize) -> Option<&[u8]> {
        if from > to || to > self.len {
            None
        } else {
            Some(&self.bytes[from..to])
        }
    }
}
