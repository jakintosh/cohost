mod instruction;
mod len;
mod register;
mod stack;

pub use instruction::Ins as Instruction;
pub use instruction::{opcode_to_str, str_to_opcode};

use instruction::Ins;
use register::Register64;
use stack::Stack;

// use self::instruction::LenF;

const STACK_FALSE: u8 = 0x00;
const DMA_COUNT: usize = 4;
const DEVICE_COUNT: usize = 16;

trait Push {
    fn push(&mut self, bytes: &[u8]);
}
trait Pop {
    fn pop(&self, len: usize) -> &[u8];
}

#[derive(Copy, Clone)]
pub struct DeviceSlot {
    pub status_reg: u8,
    pub vector: u16,
    pub identifier: [u8; 32],
    pub in_buffer: [u8; 64],
    pub out_buffer: [u8; 64],
}
impl DeviceSlot {
    pub const SEND_FLAG: u8 = 0b1000_0000;
    pub const DONE_FLAG: u8 = 0b0100_0000;
    pub const BLOCK_FLAG: u8 = 0b0010_0000;

    pub fn new() -> DeviceSlot {
        DeviceSlot {
            status_reg: 0,
            vector: 0,
            identifier: [0; 32],
            in_buffer: [0; 64],
            out_buffer: [0; 64],
        }
    }
}

#[derive(Copy, Clone)]
pub struct DMA {
    pub status_reg: u8,
    pub address: u32,
    pub buffer_len: u32,
    pub payload_len: u32,
}
impl DMA {
    pub const REQ_BIT: u8 = 0b1000_0000;

    pub fn new() -> DMA {
        DMA {
            status_reg: 0,
            address: 0,
            buffer_len: 0,
            payload_len: 0,
        }
    }
}

pub struct CPU {
    pub program_counter: u16,
    pub memory_address: u64,

    pub hold_reg: Register64,
    pub data_st: Stack,
    pub swap_st: Stack,
    pub return_st: Stack,

    pub memory: [u8; 65_536],

    pub slot_mask: u16,
    pub devices: [DeviceSlot; DEVICE_COUNT],
    pub dma_controllers: [DMA; DMA_COUNT],
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            program_counter: 0,
            memory_address: 0,

            hold_reg: Register64::new(),
            data_st: Stack::new(),
            swap_st: Stack::new(),
            return_st: Stack::new(),

            memory: [0; 65_536],
            dma_controllers: [DMA::new(); DMA_COUNT],

            slot_mask: 0,
            devices: [DeviceSlot::new(); DEVICE_COUNT],
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let rom_len = rom.len();
        if rom_len > self.memory.len() {
            panic!("trying to lod rom too big for memory")
        }
        self.memory[0..rom_len].copy_from_slice(&rom[..]);
    }
    pub fn connect_device(&mut self, identifier: [u8; 32]) {
        fn get_free_slot(mask: u16) -> Option<usize> {
            for i in 0..DEVICE_COUNT {
                if 1 << i & mask == 0 {
                    return Some(i);
                }
            }
            None
        }

        let Some(slot) = get_free_slot(self.slot_mask) else {
            eprintln!("device list full");
            return;
        };

        let device = &mut self.devices[slot];
        device.identifier = identifier;
    }

    pub fn interrupt(&mut self, address: u16) {
        self.return_st.push(&self.program_counter.to_le_bytes());
        self.program_counter = le_slice_to_u16(&address.to_le_bytes());
    }

    pub fn execute(&mut self) {
        let byte = self.memory[self.program_counter as usize];
        let instruction = Ins::from(byte);
        match instruction {
            Ins::NoOperation => {}

            // stack movement
            Ins::DuplicateData { len } => self.data_st.duplicate(len as usize),
            Ins::CopyDataToSwap { len } => self.swap_st.push(self.data_st.pop(len as usize)),
            Ins::CopyDataToReturn { len } => self.return_st.push(self.data_st.pop(len as usize)),
            Ins::CopyDataToHold { len } => self.hold_reg.push(self.data_st.pop(len as usize)),
            Ins::CopySwapToData { len } => self.data_st.push(self.swap_st.pop(len as usize)),
            Ins::DuplicateSwap { len } => self.swap_st.duplicate(len as usize),
            Ins::CopySwapToReturn { len } => self.return_st.push(self.swap_st.pop(len as usize)),
            Ins::CopySwapToHold { len } => self.hold_reg.push(self.swap_st.pop(len as usize)),
            Ins::CopyReturnToData { len } => self.data_st.push(self.return_st.pop(len as usize)),
            Ins::CopyReturnToSwap { len } => self.swap_st.push(self.return_st.pop(len as usize)),
            Ins::DuplicateReturn { len } => self.return_st.duplicate(len as usize),
            Ins::CopyReturnToHold { len } => self.hold_reg.push(self.return_st.pop(len as usize)),
            Ins::CopyHoldToData { len } => self.data_st.push(self.hold_reg.pop(len as usize)),
            Ins::CopyHoldToSwap { len } => self.swap_st.push(self.hold_reg.pop(len as usize)),
            Ins::CopyHoldToReturn { len } => self.return_st.push(self.hold_reg.pop(len as usize)),
            Ins::DropData => self.data_st.drop(1),
            Ins::DropSwap => self.swap_st.drop(1),
            Ins::DropReturn => self.return_st.drop(1),

            // branching
            Ins::Jump {
                len,
                con: conditional,
                rel: relative,
            } => {
                if conditional {
                    let condition = le_slice_to_u8(self.data_st.pop(1));
                    self.data_st.drop(1);
                    if condition == STACK_FALSE {
                        return; // don't execute the jump
                    };
                }

                let value = self.pop_operand16(len as usize);
                self.program_counter = match relative {
                    true => self.program_counter + value,
                    false => value,
                };

                return; // avoid default PC increment
            }
            Ins::Call { len } => {
                self.return_st.push(&self.program_counter.to_le_bytes());
                self.program_counter = self.pop_operand16(len as usize);
                return; // avoid default PC increment
            }
            Ins::Return { len } => {
                self.program_counter = le_slice_to_u16(self.return_st.pop(len as usize));
                self.return_st.drop(len as usize);
                return; // avoid default PC increment
            }

            // accessing memory
            Ins::Literal { len } => {
                let lit_range = self.get_lit_range(len as usize);
                self.data_st.push(&self.memory[lit_range]);
                self.program_counter += len as u16; // skip consumed literal
            }
            Ins::Address { len } => {
                self.memory_address = self.pop_operand64(len as usize);
                if self.memory_address as usize > self.memory.len() {
                    panic!("Memory Overflow")
                }
            }
            Ins::Store { len } => {
                let start = self.memory_address as usize;
                let end = self.memory_address as usize + len as usize;
                if end > self.memory.len() {
                    panic!("Memory Overflow")
                }
                let data = self.data_st.pop(len as usize);
                self.memory[start..end].copy_from_slice(data);
            }
            Ins::Load { len } => {
                let range =
                    self.memory_address as usize..self.memory_address as usize + len as usize;
                let data = &self.memory[range];
                self.data_st.push(data);
            }

            // working with DMA
            Ins::DMARead => {
                let index = self.pop_operand8();
                let dma = &self.dma_controllers[index as usize];
                let length = dma.buffer_len.to_le_bytes();
                let address = dma.address.to_le_bytes();
                self.data_st.push(&length);
                self.data_st.push(&address);
            }
            Ins::DMAWrite { len } => {
                let (index, flag) = self.pop_operands8();
                let (address, length) = self.pop_operands32(len as usize);
                let dma = &mut self.dma_controllers[index as usize];
                dma.status_reg |= flag;
                dma.address = address;
                dma.buffer_len = length;
            }
            Ins::DMAPoll => {
                let (index, flag) = self.pop_operands8();
                let dma = &self.dma_controllers[index as usize];
                let flag_set = (dma.status_reg & flag) != 0;
                self.push_result_bool(flag_set);
            }

            // working with devices
            Ins::DeviceRead { len } => {
                // data ( index8, offset8 -- value )
                let (index, offset) = self.pop_operands8();
                let slot = &self.devices[index as usize];
                let value = &slot.in_buffer[offset as usize..(offset) as usize + len as usize];
                self.data_st.push(&value);
            }
            Ins::DeviceWrite { len } => {
                // data ( index8, flag8, offset8, valueLEN -- )
                let (index, flag) = self.pop_operands8();
                let offset = self.pop_operand8() as usize;
                let value = self.data_st.pop(len as usize);
                let range = offset..offset + len as usize;
                let slot = &mut self.devices[index as usize];
                slot.status_reg |= flag;
                slot.out_buffer[range].copy_from_slice(value);
            }
            Ins::DevicePoll { len } => {
                // data ( index8, addressLEN -- ) | memory { [address] => device.identifier }
                let index = self.pop_operand8();
                let address = self.pop_operand32(len as usize);
                let slot = &self.devices[index as usize];
                let range = address as usize..address as usize + 32;
                self.memory[range].copy_from_slice(&slot.identifier);
            }

            // arithmetic
            Ins::Add { len } => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len as usize);
                self.push_result64(len as usize, lhs + rhs)
            }
            Ins::Subtract { len } => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len as usize);
                self.push_result64(len as usize, lhs - rhs);
            }
            Ins::Multiply { len } => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len as usize);
                self.push_result64(len as usize, lhs * rhs);
            }
            Ins::Divide { len } => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len as usize);
                self.push_result64(len as usize, lhs / rhs);
            }

            // comparisons
            Ins::Greater { len } => {
                // data ( lhsLEN, rhsLEN -- result8)
                let (lhs, rhs) = self.pop_operands64(len as usize);
                self.push_result_bool(lhs > rhs);
            }
            Ins::Less { len } => {
                // data ( lhsLEN, rhsLEN -- result8)
                let (lhs, rhs) = self.pop_operands64(len as usize);
                self.push_result_bool(lhs < rhs);
            }
            Ins::Equal { len } => {
                // data ( lhsLEN, rhsLEN -- result8)
                let (lhs, rhs) = self.pop_operands64(len as usize);
                self.push_result_bool(lhs == rhs);
            }
            Ins::NotEqual { len } => {
                // data ( lhsLEN, rhsLEN -- result8)
                let (lhs, rhs) = self.pop_operands64(len as usize);
                self.push_result_bool(lhs != rhs);
            }

            // float arithmetic
            Ins::AddF { len } => match len {
                len::LenF::L32 => {
                    let (lhs, rhs) = self.pop_operands32(4);
                    let result = f32_from_u32(lhs) + f32_from_u32(rhs);
                    self.push_result32(len as usize, u32_from_f32(result))
                }
                len::LenF::L64 => {
                    let (lhs, rhs) = self.pop_operands64(8);
                    let result = f64_from_u64(lhs) + f64_from_u64(rhs);
                    self.push_result64(len as usize, u64_from_f64(result))
                }
            },
            Ins::SubtractF { len } => match len {
                len::LenF::L32 => {
                    let (lhs, rhs) = self.pop_operands32(4);
                    let result = f32_from_u32(lhs) - f32_from_u32(rhs);
                    self.push_result32(len as usize, u32_from_f32(result))
                }
                len::LenF::L64 => {
                    let (lhs, rhs) = self.pop_operands64(8);
                    let result = f64_from_u64(lhs) - f64_from_u64(rhs);
                    self.push_result64(len as usize, u64_from_f64(result))
                }
            },
            Ins::MultiplyF { len } => match len {
                len::LenF::L32 => {
                    let (lhs, rhs) = self.pop_operands32(4);
                    let result = f32_from_u32(lhs) * f32_from_u32(rhs);
                    self.push_result32(len as usize, u32_from_f32(result))
                }
                len::LenF::L64 => {
                    let (lhs, rhs) = self.pop_operands64(8);
                    let result = f64_from_u64(lhs) * f64_from_u64(rhs);
                    self.push_result64(len as usize, u64_from_f64(result))
                }
            },
            Ins::DivideF { len } => match len {
                len::LenF::L32 => {
                    let (lhs, rhs) = self.pop_operands32(4);
                    let result = f32_from_u32(lhs) / f32_from_u32(rhs);
                    self.push_result32(len as usize, u32_from_f32(result))
                }
                len::LenF::L64 => {
                    let (lhs, rhs) = self.pop_operands64(8);
                    let result = f64_from_u64(lhs) / f64_from_u64(rhs);
                    self.push_result64(len as usize, u64_from_f64(result))
                }
            },

            // float comparisons
            Ins::GreaterF { len } => {
                let result = match len {
                    len::LenF::L32 => {
                        let (lhs, rhs) = self.pop_operands32(4);
                        f32_from_u32(lhs) > f32_from_u32(rhs)
                    }
                    len::LenF::L64 => {
                        let (lhs, rhs) = self.pop_operands64(8);
                        f64_from_u64(lhs) > f64_from_u64(rhs)
                    }
                };
                self.push_result_bool(result)
            }
            Ins::LessF { len } => {
                let result = match len {
                    len::LenF::L32 => {
                        let (lhs, rhs) = self.pop_operands32(4);
                        f32_from_u32(lhs) < f32_from_u32(rhs)
                    }
                    len::LenF::L64 => {
                        let (lhs, rhs) = self.pop_operands64(8);
                        f64_from_u64(lhs) < f64_from_u64(rhs)
                    }
                };
                self.push_result_bool(result)
            }

            // bitwise logic
            Ins::And { len } => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len as usize);
                self.push_result64(len as usize, lhs & rhs);
            }
            Ins::Or { len } => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len as usize);
                self.push_result64(len as usize, lhs | rhs);
            }
            Ins::Xor { len } => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len as usize);
                self.push_result64(len as usize, lhs ^ rhs);
            }
            Ins::Not { len } => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let operand = self.pop_operand64(len as usize);
                self.push_result64(len as usize, !operand);
            }
            Ins::ShiftL { len } => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (shift, operand) = self.pop_operands64(len as usize);
                self.push_result64(len as usize, operand << shift);
            }
            Ins::ShiftR { len } => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (shift, operand) = self.pop_operands64(len as usize);
                self.push_result64(len as usize, operand >> shift);
            }
        }

        self.program_counter += 1;
    }

    fn get_lit_range(&self, len: usize) -> std::ops::Range<usize> {
        let start = self.program_counter as usize + 1;
        let end = start + len;
        start..end
    }

    fn pop_operand8(&mut self) -> u8 {
        let operand = le_slice_to_u8(self.data_st.pop(1));
        self.data_st.drop(1);
        operand
    }
    fn pop_operands8(&mut self) -> (u8, u8) {
        (self.pop_operand8(), self.pop_operand8())
    }

    fn pop_operand16(&mut self, len: usize) -> u16 {
        let operand = le_slice_to_u16(self.data_st.pop(len));
        self.data_st.drop(len);
        operand
    }
    fn pop_operands16(&mut self, len: usize) -> (u16, u16) {
        (self.pop_operand16(len), self.pop_operand16(len))
    }

    fn pop_operand32(&mut self, len: usize) -> u32 {
        let operand = le_slice_to_u32(self.data_st.pop(len));
        self.data_st.drop(len);
        operand
    }
    fn pop_operands32(&mut self, len: usize) -> (u32, u32) {
        (self.pop_operand32(len), self.pop_operand32(len))
    }

    fn pop_operand64(&mut self, len: usize) -> u64 {
        let operand = le_slice_to_u64(self.data_st.pop(len));
        self.data_st.drop(len);
        operand
    }
    fn pop_operands64(&mut self, len: usize) -> (u64, u64) {
        (self.pop_operand64(len), self.pop_operand64(len))
    }

    fn push_result_bool(&mut self, result: bool) {
        self.push_result8(match result {
            true => 0xff,
            false => STACK_FALSE,
        });
    }
    fn push_result8(&mut self, result: u8) {
        self.data_st.push(&result.to_le_bytes());
    }
    fn push_result32(&mut self, len: usize, result: u32) {
        self.data_st.push(&result.to_le_bytes()[0..len]);
    }
    fn push_result64(&mut self, len: usize, result: u64) {
        self.data_st.push(&result.to_le_bytes()[0..len]);
    }
}

fn le_slice_to_u8(slice: &[u8]) -> u8 {
    slice[0].clone()
}
fn le_slice_to_u16(slice: &[u8]) -> u16 {
    let mut result = 0u16;

    let max = usize::min(2, slice.len());
    for i in 0..max {
        let byte = (slice[i]) as u16;
        let chunk = byte << (i * 8);
        result = result | chunk;
    }

    result
}
fn le_slice_to_u32(slice: &[u8]) -> u32 {
    let mut result = 0u32;

    let max = usize::min(4, slice.len());
    for i in 0..max {
        let byte = (slice[i]) as u32;
        let chunk = byte << (i * 8);
        result = result | chunk;
    }

    result
}
fn le_slice_to_u64(slice: &[u8]) -> u64 {
    let mut result = 0u64;

    let max = usize::min(8, slice.len());
    for i in 0..max {
        let byte = (slice[i]) as u64;
        let chunk = byte << (i * 8);
        result = result | chunk;
    }

    result
}

fn f32_from_u32(bytes: u32) -> f32 {
    f32::from_le_bytes(bytes.to_le_bytes())
}
fn u32_from_f32(bytes: f32) -> u32 {
    u32::from_le_bytes(bytes.to_le_bytes())
}

fn f64_from_u64(bytes: u64) -> f64 {
    f64::from_le_bytes(bytes.to_le_bytes())
}
fn u64_from_f64(bytes: f64) -> u64 {
    u64::from_le_bytes(bytes.to_le_bytes())
}
