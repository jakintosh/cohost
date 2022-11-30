use std::io::Read;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Devices {
    Console,
    Other([u8; 32]),
}
impl From<Devices> for [u8; 32] {
    fn from(device: Devices) -> Self {
        let mut buffer = [0u8; 32];
        match device {
            Devices::Console => buffer[0] = 0,
            Devices::Other(id) => buffer.copy_from_slice(&id[..]),
        };
        buffer
    }
}
impl From<[u8; 32]> for Devices {
    fn from(array: [u8; 32]) -> Self {
        for i in 1..32 {
            if array[i] != 0 {
                return Devices::Other(array);
            }
        }
        match array[0] {
            0x00 => Devices::Console,
            _ => Devices::Other(array),
        }
    }
}
pub trait Device {
    fn poll(&mut self) -> Option<[u8; 64]>;
    fn recv(&mut self, buffer: &[u8; 64]);
}

// console device
pub(crate) struct Console {
    read_cursor: Option<usize>,
    read_buffer: Vec<u8>,
}
impl Console {
    pub fn new() -> Console {
        Console {
            read_cursor: None,
            read_buffer: Vec::new(),
        }
    }
}
impl Device for Console {
    fn poll(&mut self) -> Option<[u8; 64]> {
        fn read_buffer(cursor: usize, in_buffer: &Vec<u8>) -> (Option<usize>, [u8; 64]) {
            // advance cursor
            let end = usize::min(cursor + 64, in_buffer.len());
            let copy_size = end - cursor;
            let next_cursor = match end == in_buffer.len() {
                true => None,
                false => Some(end),
            };

            // copy from stored buffer
            let mut out_buffer = [0; 64];
            out_buffer[0..copy_size].copy_from_slice(&in_buffer[(cursor..end)]);

            (next_cursor, out_buffer)
        }

        // if we have cursor, use it, otherwise poll stdin
        let cursor = match self.read_cursor {
            Some(c) => c,
            None => {
                self.read_buffer = std::io::stdin().bytes().map(|b| b.unwrap()).collect();
                match self.read_buffer.len() {
                    0 => return None,
                    _ => 0,
                }
            }
        };

        let (cursor, poll_buffer) = read_buffer(cursor, &self.read_buffer);
        self.read_cursor = cursor;
        Some(poll_buffer)
    }
    fn recv(&mut self, buffer: &[u8; 64]) {
        print!("{}", String::from_utf8_lossy(buffer))
    }
}
