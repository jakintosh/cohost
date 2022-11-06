use std::ops::Range;

use super::{Pop, Push};

pub(crate) struct Stack {
    pointer: usize,
    buffer: [u8; 256],
}
impl Stack {
    pub fn new() -> Stack {
        Stack {
            pointer: 0,
            buffer: [0; 256],
        }
    }
    pub fn push_byte(&mut self, byte: u8) {
        self.buffer[self.pointer] = byte;
        self.pointer += 1;
        if self.pointer >= 256 {
            panic!("Stack Overflow")
        }
    }
    pub fn duplicate(&mut self, len: usize) {
        let top = self.top_range(len);
        self.buffer.copy_within(top, self.pointer + 1);
        self.pointer += len;
    }
    pub fn drop(&mut self, len: usize) {
        if len > self.pointer {
            panic!("Stack Underflow");
        }
        self.pointer -= len;
    }
    pub fn top_byte(&mut self) -> u8 {
        if self.pointer == 0 {
            panic!("Stack Underflow")
        }
        self.buffer[self.pointer - 1]
    }

    pub fn get_slice(&self, range: Range<usize>) -> &[u8] {
        &self.buffer[range]
    }
    pub fn make_range(&self, len: usize, pos: isize) -> Range<usize> {
        let offset = pos * len as isize;
        let end = self.pointer + offset as usize;
        let start = end - len;

        start..end
    }
    pub fn top_range(&self, len: usize) -> Range<usize> {
        self.make_range(len, 0)
    }
    pub fn next_range(&self, len: usize) -> Range<usize> {
        self.make_range(len, -1)
    }
}

impl Push for Stack {
    fn push(&mut self, bytes: &[u8]) {
        let len = bytes.len();
        let range = self.next_range(len);
        self.buffer[range].copy_from_slice(bytes);
        self.pointer += len;
        if self.pointer >= 256 {
            panic!("Stack Overflow");
        }
    }
}

impl Pop for Stack {
    fn top(&self, len: usize) -> &[u8] {
        self.get_slice(self.top_range(len))
    }
}
