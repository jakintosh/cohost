mod register;
mod stack;

use register::Register64;
use stack::Stack;

const STACK_FALSE: u8 = 0x00;
const DMA_COUNT: usize = 4;
const DEVICE_COUNT: usize = 16;

trait Push {
    fn push(&mut self, bytes: &[u8]);
}
trait Pop {
    fn pop(&self, len: usize) -> &[u8];
}

enum Instruction {
    CopyDataToData,
    CopyDataToSwap,
    CopyDataToReturn,
    CopyDataToHold,
    CopySwapToData,
    CopySwapToSwap,
    CopySwapToReturn,
    CopySwapToHold,
    CopyReturnToData,
    CopyReturnToSwap,
    CopyReturnToReturn,
    CopyReturnToHold,
    CopyHoldToData,
    CopyHoldToSwap,
    CopyHoldToReturn,
    Literal,

    Jump { conditional: bool, relative: bool },
    Call,
    Return,

    Address,
    Store,
    Load,

    DMARead,
    DMAWrite,
    DMAPoll,

    DeviceRead,
    DeviceWrite,
    DevicePoll,

    Add,
    Subtract,
    Multiply,
    Divide,

    Greater,
    Less,
    Equal,
    NotEqual,

    And,
    Or,
    Xor,
    Not,
    ShiftL,
    ShiftR,

    DropData,
    DropSwap,
    DropReturn,
}
impl From<u8> for Instruction {
    fn from(byte: u8) -> Self {
        let byte = 0b00111111 & byte;
        match byte {
            // stack movement
            0x00 => Self::CopyDataToData,
            0x01 => Self::CopyDataToSwap,
            0x02 => Self::CopyDataToReturn,
            0x03 => Self::CopyDataToHold,
            0x04 => Self::CopySwapToData,
            0x05 => Self::CopySwapToSwap,
            0x06 => Self::CopySwapToReturn,
            0x07 => Self::CopySwapToHold,
            0x08 => Self::CopyReturnToData,
            0x09 => Self::CopyReturnToSwap,
            0x0a => Self::CopyReturnToReturn,
            0x0b => Self::CopyReturnToHold,
            0x0c => Self::CopyHoldToData,
            0x0d => Self::CopyHoldToSwap,
            0x0e => Self::CopyHoldToReturn,
            0x0f => Self::Literal,

            // branching
            0x10 => Self::Jump {
                conditional: false,
                relative: false,
            },
            0x11 => Self::Jump {
                conditional: true,
                relative: false,
            },
            0x12 => Self::Jump {
                conditional: false,
                relative: true,
            },
            0x13 => Self::Jump {
                conditional: true,
                relative: true,
            },
            0x14 => Self::Call,
            0x15 => Self::Return,

            // memory
            0x16 => Self::Address,
            0x17 => Self::Store,
            0x18 => Self::Load,

            // DMA
            0x19 => Self::DMARead,
            0x1a => Self::DMAWrite,
            0x1b => Self::DMAPoll,

            // Devices
            0x1c => Self::DeviceRead,
            0x1d => Self::DeviceWrite,
            0x1e => Self::DevicePoll,

            // logic and arithmetic
            0x20 => Self::Add,
            0x21 => Self::Subtract,
            0x22 => Self::Multiply,
            0x23 => Self::Divide,
            0x24 => Self::Greater,
            0x25 => Self::Less,
            0x26 => Self::Equal,
            0x27 => Self::NotEqual,
            // 0x28 => Self::AddF,
            // 0x29 => Self::SubtractF,
            // 0x2a => Self::MultiplyF,
            // 0x2b => Self::DivideF,
            // 0x2c => Self::GreaterF,
            // 0x2d => Self::LessF,
            // 0x2e => Self::EqualF,
            // 0x2f => Self::NotEqualF,

            // bitwise
            0x30 => Self::And,
            0x31 => Self::Or,
            0x32 => Self::Xor,
            0x33 => Self::Not,
            0x34 => Self::ShiftL,
            0x35 => Self::ShiftR,

            // dropping
            0x36 => Self::DropData,
            0x37 => Self::DropSwap,
            0x38 => Self::DropReturn,

            _ => panic!("invalid instruction"),
        }
    }
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

pub(crate) struct CPU {
    pub program_counter: u32,
    pub memory_address: u32,

    pub hold_reg: Register64,
    pub data_st: Stack,
    pub swap_st: Stack,
    pub return_st: Stack,

    pub memory: [u8; 65_536],
    pub dma_controllers: [DMA; DMA_COUNT],
    pub slot_mask: u16,
    pub devices: [DeviceSlot; DEVICE_COUNT],
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
        self.program_counter = CPU::le_slice_to_u32(&address.to_le_bytes());
    }

    pub fn execute(&mut self) {
        let instruction: u8 = self.memory[self.program_counter as usize];
        let len = {
            let masked_size = instruction & 0b11000000;
            let num_shifts = (masked_size) >> 6;
            let num_bytes = 0b00000001 << num_shifts;
            num_bytes
        };
        match instruction.into() {
            // stack movement
            Instruction::CopyDataToData => self.data_st.duplicate(len),
            Instruction::CopyDataToSwap => self.swap_st.push(self.data_st.pop(len)),
            Instruction::CopyDataToReturn => self.return_st.push(self.data_st.pop(len)),
            Instruction::CopyDataToHold => self.hold_reg.push(self.data_st.pop(len)),
            Instruction::CopySwapToData => self.data_st.push(self.swap_st.pop(len)),
            Instruction::CopySwapToSwap => self.swap_st.duplicate(len),
            Instruction::CopySwapToReturn => self.return_st.push(self.swap_st.pop(len)),
            Instruction::CopySwapToHold => self.hold_reg.push(self.swap_st.pop(len)),
            Instruction::CopyReturnToData => self.data_st.push(self.return_st.pop(len)),
            Instruction::CopyReturnToSwap => self.swap_st.push(self.return_st.pop(len)),
            Instruction::CopyReturnToReturn => self.return_st.duplicate(len),
            Instruction::CopyReturnToHold => self.hold_reg.push(self.return_st.pop(len)),
            Instruction::CopyHoldToData => self.data_st.push(self.hold_reg.pop(len)),
            Instruction::CopyHoldToSwap => self.swap_st.push(self.hold_reg.pop(len)),
            Instruction::CopyHoldToReturn => self.return_st.push(self.hold_reg.pop(len)),
            Instruction::Literal => {
                let lit_range = self.get_lit_range(len);
                self.data_st.push(&self.memory[lit_range]);
                self.program_counter += 1 + len as u32;
                return; // avoid default PC increment
            }

            // branching
            Instruction::Jump {
                conditional,
                relative,
            } => {
                if conditional {
                    let condition = self.data_st.top_byte();
                    self.data_st.drop(1);
                    if condition == STACK_FALSE {
                        return; // don't execute the jump
                    };
                }

                let value = self.pop_operand32(len);
                self.program_counter = match relative {
                    true => self.program_counter + value,
                    false => value,
                };

                return; // avoid default PC increment
            }
            Instruction::Call => {
                self.return_st.push(&self.program_counter.to_le_bytes());
                self.program_counter = self.pop_operand32(len);
                return; // avoid default PC increment
            }
            Instruction::Return => {
                self.program_counter = CPU::le_slice_to_u32(self.return_st.pop(len));
                self.return_st.drop(len);
                return; // avoid default PC increment
            }

            // accessing memory
            Instruction::Address => {
                self.memory_address = self.pop_operand32(len);
            }
            Instruction::Store => {
                let data = self.data_st.pop(len);
                let range = self.memory_address as usize..self.memory_address as usize + len;
                self.memory[range].copy_from_slice(data);
            }
            Instruction::Load => {
                let range = self.memory_address as usize..self.memory_address as usize + len;
                let data = &self.memory[range];
                self.data_st.push(data);
            }

            // working with DMA
            Instruction::DMARead => {
                let index = self.pop_operand8();
                let dma = &self.dma_controllers[index as usize];
                let length = dma.buffer_len.to_le_bytes();
                let address = dma.address.to_le_bytes();
                self.data_st.push(&length);
                self.data_st.push(&address);
            }
            Instruction::DMAWrite => {
                let (index, flag) = self.pop_operands8();
                let (address, length) = self.pop_operands32(len);
                let dma = &mut self.dma_controllers[index as usize];
                dma.status_reg |= flag;
                dma.address = address;
                dma.buffer_len = length;
            }
            Instruction::DMAPoll => {
                let (index, flag) = self.pop_operands8();
                let dma = &self.dma_controllers[index as usize];
                let flag_set = (dma.status_reg & flag) != 0;
                self.push_result_bool(flag_set);
            }

            // working with devices
            Instruction::DeviceRead => {
                // data ( index8, offset8 -- value )
                let (index, offset) = self.pop_operands8();
                let slot = &self.devices[index as usize];
                let value = &slot.in_buffer[offset as usize..(offset) as usize + len];
                self.data_st.push(&value);
            }
            Instruction::DeviceWrite => {
                // data ( index8, flag8, offset8, valueLEN -- )
                let (index, flag) = self.pop_operands8();
                let offset = self.pop_operand8() as usize;
                let value = self.data_st.pop(len);
                let range = offset..offset + len;
                let slot = &mut self.devices[index as usize];
                slot.status_reg |= flag;
                slot.out_buffer[range].copy_from_slice(value);
            }
            Instruction::DevicePoll => {
                // data ( index8, addressLEN -- ) | memory { [address] => device.identifier }
                let index = self.pop_operand8();
                let address = self.pop_operand32(len);
                let slot = &self.devices[index as usize];
                let range = address as usize..address as usize + 32;
                self.memory[range].copy_from_slice(&slot.identifier);
            }

            // arithmetic
            Instruction::Add => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs + rhs);
            }
            Instruction::Subtract => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs - rhs);
            }
            Instruction::Multiply => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs * rhs);
            }
            Instruction::Divide => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs / rhs);
            }

            // comparisons
            Instruction::Greater => {
                // data ( lhsLEN, rhsLEN -- result8)
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result_bool(lhs > rhs);
            }
            Instruction::Less => {
                // data ( lhsLEN, rhsLEN -- result8)
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result_bool(lhs < rhs);
            }
            Instruction::Equal => {
                // data ( lhsLEN, rhsLEN -- result8)
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result_bool(lhs == rhs);
            }
            Instruction::NotEqual => {
                // data ( lhsLEN, rhsLEN -- result8)
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result_bool(lhs != rhs);
            }

            // bitwise logic
            Instruction::And => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs & rhs);
            }
            Instruction::Or => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs | rhs);
            }
            Instruction::Xor => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs ^ rhs);
            }
            Instruction::Not => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let operand = self.pop_operand64(len);
                self.push_result64(len, !operand);
            }
            Instruction::ShiftL => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (shift, operand) = self.pop_operands64(len);
                self.push_result64(len, operand << shift);
            }
            Instruction::ShiftR => {
                // data ( lhsLEN, rhsLEN -- resultLEN)
                let (shift, operand) = self.pop_operands64(len);
                self.push_result64(len, operand >> shift);
            }

            // dropping stacks
            Instruction::DropData => self.data_st.drop(len),
            Instruction::DropSwap => self.swap_st.drop(len),
            Instruction::DropReturn => self.return_st.drop(len),
        }

        self.program_counter += 1;
    }

    fn le_slice_to_u32(slice: &[u8]) -> u32 {
        let mut result = 0u32;

        for i in 0..4 {
            let byte = (slice[i]) as u32;
            let chunk = byte << (i * 8);
            result = result | chunk;
        }

        result
    }
    fn le_slice_to_u64(slice: &[u8]) -> u64 {
        let mut result = 0u64;

        for i in 0..8 {
            let byte = (slice[i]) as u64;
            let chunk = byte << (i * 8);
            result = result | chunk;
        }

        result
    }

    fn get_lit_range(&self, len: usize) -> std::ops::Range<usize> {
        let start = self.program_counter as usize + 1;
        let end = start + len;
        start..end
    }

    fn pop_operand8(&mut self) -> u8 {
        let operand = self.data_st.top_byte();
        self.data_st.drop(1);
        operand
    }
    fn pop_operands8(&mut self) -> (u8, u8) {
        (self.pop_operand8(), self.pop_operand8())
    }

    fn pop_operand32(&mut self, len: usize) -> u32 {
        let operand = CPU::le_slice_to_u32(self.data_st.pop(len));
        self.data_st.drop(len);
        operand
    }
    fn pop_operands32(&mut self, len: usize) -> (u32, u32) {
        (self.pop_operand32(len), self.pop_operand32(len))
    }

    fn pop_operand64(&mut self, len: usize) -> u64 {
        let operand = CPU::le_slice_to_u64(self.data_st.pop(len));
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
        self.data_st.push_byte(result);
    }
    fn push_result32(&mut self, len: usize, result: u32) {
        self.data_st.push(&result.to_le_bytes()[0..len]);
    }
    fn push_result64(&mut self, len: usize, result: u64) {
        self.data_st.push(&result.to_le_bytes()[0..len]);
    }
}
