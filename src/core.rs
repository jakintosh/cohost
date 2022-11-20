mod register;
mod stack;

use register::Register64;
use stack::Stack;

const STACK_FALSE: u8 = 0x00;

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
    DMASet,
    DMACheck,
    DropData,
    DropSwap,
    DropReturn,
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Xor,
    Not,
    ShiftL,
    ShiftR,
    Greater,
    Less,
    Equal,
    NotEqual,
}
impl From<u8> for Instruction {
    fn from(byte: u8) -> Self {
        let byte = 0b00111111 & byte;
        match byte {
            // data movement
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

            // jumps, memory, and registers
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
            0x16 => Self::Address,
            0x17 => Self::Store,
            0x18 => Self::Load,
            0x19 => Self::DMASet,
            0x1a => Self::DMACheck,
            0x1b => Self::DropData,
            0x1c => Self::DropSwap,
            0x1d => Self::DropReturn,

            // logic and arithmetic
            0x20 => Self::Add,
            0x21 => Self::Subtract,
            0x22 => Self::Multiply,
            0x23 => Self::Divide,
            0x24 => Self::And,
            0x25 => Self::Or,
            0x26 => Self::Xor,
            0x27 => Self::Not,
            0x28 => Self::ShiftL,
            0x29 => Self::ShiftR,
            0x2a => Self::Greater,
            0x2b => Self::Less,
            0x2c => Self::Equal,
            0x2d => Self::NotEqual,

            _ => panic!("invalid instruction"),
        }
    }
}

pub struct DMA {
    pub status_reg: u8,
    pub address: u32,
    pub length: u32,
}
impl DMA {
    pub const REQUEST_BIT: u8 = 0x80;
    pub const RESPONSE_BIT: u8 = 0x40;

    pub fn new() -> DMA {
        DMA {
            status_reg: 0,
            address: 0,
            length: 0,
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
    pub dma_controller: DMA,
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
            dma_controller: DMA::new(),
        }
    }
    pub fn execute(&mut self) {
        let instruction: u8 = self.memory[self.program_counter as usize];
        let len = CPU::parse_bytes(instruction);
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
            Instruction::DMASet => {
                let flag = self.pop_operand8();
                let (address, length) = self.pop_operands32(len);
                self.dma_controller.status_reg |= flag;
                self.dma_controller.address = address;
                self.dma_controller.length = length;
            }
            Instruction::DMACheck => {
                let flag = self.pop_operand8();
                let flag_set = (self.dma_controller.status_reg & flag) != 0;
                self.push_result_bool(flag_set);
            }

            // dropping stacks
            Instruction::DropData => self.data_st.drop(len),
            Instruction::DropSwap => self.swap_st.drop(len),
            Instruction::DropReturn => self.return_st.drop(len),

            // arithmetic
            Instruction::Add => {
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs + rhs);
            }
            Instruction::Subtract => {
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs - rhs);
            }
            Instruction::Multiply => {
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs * rhs);
            }
            Instruction::Divide => {
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs / rhs);
            }

            // bitwise logic
            Instruction::And => {
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs & rhs);
            }
            Instruction::Or => {
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs | rhs);
            }
            Instruction::Xor => {
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result64(len, lhs ^ rhs);
            }
            Instruction::Not => {
                let operand = self.pop_operand64(len);
                self.push_result64(len, !operand);
            }
            Instruction::ShiftL => {
                let (shift, operand) = self.pop_operands64(len);
                self.push_result64(len, operand << shift);
            }
            Instruction::ShiftR => {
                let (shift, operand) = self.pop_operands64(len);
                self.push_result64(len, operand >> shift);
            }

            // comparisons
            Instruction::Greater => {
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result_bool(lhs > rhs);
            }
            Instruction::Less => {
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result_bool(lhs < rhs);
            }
            Instruction::Equal => {
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result_bool(lhs == rhs);
            }
            Instruction::NotEqual => {
                let (lhs, rhs) = self.pop_operands64(len);
                self.push_result_bool(lhs != rhs);
            }
        }

        self.program_counter += 1;
    }
    pub fn run(&mut self) {
        loop {
            self.execute()
        }
    }

    fn parse_bytes(instruction: u8) -> usize {
        let masked_size = instruction & 0b11000000;
        let num_shifts = (masked_size) >> 6;
        let num_bytes = 0b00000001 << num_shifts;

        num_bytes
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
