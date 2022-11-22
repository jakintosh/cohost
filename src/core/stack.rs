use super::{Pop, Push};

const STACK_SIZE: usize = 256;

pub(crate) struct Stack {
    pointer: usize,
    buffer: [u8; STACK_SIZE],
}
impl Stack {
    pub fn new() -> Stack {
        Stack {
            pointer: 0,
            buffer: [0; STACK_SIZE],
        }
    }
    pub fn len(&self) -> usize {
        self.pointer
    }
    pub fn duplicate(&mut self, len: usize) {
        if self.pointer < len {
            panic!("Stack Underflow");
        }
        if self.pointer + len > STACK_SIZE {
            panic!("Stack Overflow")
        }

        let top_range = self.pointer - len..self.pointer;
        self.buffer.copy_within(top_range, self.pointer);
        self.pointer += len;
    }
    pub fn drop(&mut self, len: usize) {
        if self.pointer < len {
            panic!("Stack Underflow");
        }

        self.pointer -= len;
    }
}

impl Push for Stack {
    fn push(&mut self, bytes: &[u8]) {
        let start = self.pointer;
        let end = self.pointer + bytes.len();
        if end > STACK_SIZE {
            panic!("Stack Overflow")
        }

        self.buffer[start..end].copy_from_slice(bytes);
        self.pointer = end;
    }
}

impl Pop for Stack {
    fn pop(&self, len: usize) -> &[u8] {
        if self.pointer < len {
            panic!("Stack Underflow");
        }

        &self.buffer[self.pointer - len..self.pointer]
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X?}", &self.buffer[0..self.pointer])
    }
}
