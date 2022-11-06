use super::{Pop, Push};

pub struct Register64 {
    buffer: [u8; 8],
}

impl Register64 {
    pub fn new() -> Register64 {
        Register64 { buffer: [0; 8] }
    }
}

impl Push for Register64 {
    fn push(&mut self, bytes: &[u8]) {
        let len = bytes.len();

        if len > 8 {
            panic!("Register Overflow")
        }

        for i in 0..8 {
            if i < len {
                self.buffer[i] = bytes[i];
            } else {
                self.buffer[i] = 0;
            }
        }
    }
}

impl Pop for Register64 {
    fn top(&self, len: usize) -> &[u8] {
        &self.buffer[0..len]
    }
}
