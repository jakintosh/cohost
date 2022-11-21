use std::io::{BufReader, Read};

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
struct Cursor {
    pos: usize,
    len: usize,
}
pub(crate) struct Console {
    std_in: std::io::Stdin,
    read_cursor: Option<Cursor>,
    read_buffer: Vec<u8>,
}
impl Console {
    pub fn new() -> Console {
        Console {
            std_in: std::io::stdin(),
            read_cursor: None,
            read_buffer: Vec::new(),
        }
    }
}
impl Device for Console {
    fn poll(&mut self) -> Option<[u8; 64]> {
        fn read_buffer(cursor: &Cursor, in_buffer: &Vec<u8>) -> (Option<Cursor>, [u8; 64]) {
            // advance cursor
            let start = cursor.pos;
            let end = usize::min(cursor.pos + 64, cursor.len);
            let copy_size = end - start;
            let cursor = match end == cursor.len {
                false => Some(Cursor {
                    pos: end,
                    len: cursor.len,
                }),
                true => None,
            };

            // copy from stored buffer
            let mut out_buffer = [0; 64];
            out_buffer[0..copy_size].copy_from_slice(&in_buffer[(start..end)]);

            (cursor, out_buffer)
        }

        match &self.read_cursor {
            Some(cursor) => {
                let (cursor, x) = read_buffer(cursor, &self.read_buffer);
                self.read_cursor = cursor;
                Some(x)
            }
            None => {
                let mut reader = BufReader::new(self.std_in.lock());
                match reader.read_to_end(&mut self.read_buffer) {
                    Ok(len) => {
                        let (cursor, x) = read_buffer(&Cursor { pos: 0, len }, &self.read_buffer);
                        self.read_cursor = cursor;
                        Some(x)
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    }
                }
            }
        }
    }
    fn recv(&mut self, buffer: &[u8; 64]) {
        print!("{}", String::from_utf8_lossy(buffer))
    }
}
