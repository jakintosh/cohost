/*
    0000_0000 -- ( 0 / 1 )
    0000_0001 -- ( 0 / 1 )
    0000_001X -- ( 0 / 2 )

    0000_01XX -- Call / Return ( 4 / 4 )
           DL -- L = len, D = id

    0000_1XXX -- Jump ( 8 / 8 )
          CRL -- L = len, C = cond?, R = rel?

    0001_XXXX -- Float Operatins ( 12 / 16 )
         0XXX -- Float Arithmetic (8 / 8 )
          DDL -- L = len, D = id
         1XXX -- Float Comparisons ( 4 / 8 )
          DDL -- L = len, D = id

    001X_XXXX -- Unused ( 0 / 32 )

    01XX_XXXX -- Byte Manipulation ( 56 / 64 )
      1X_XXXX -- Integer Operations ( 32 / 32 )
       1_XXXX -- Int Arithmetic ( 16 / 16 )
         DDLL -- L = len, D = id
       0_XXXX -- Int Comparisons ( 16 / 16 )
         DDLL -- L = len, D = id
      0X_XXXX -- Bitwise Ops ( 24 / 32 )
       D_DDLL -- L = len, D = id

    1XXX_XXXX -- Data Operations ( 97 / 128 )

     1XX_XXXX -- Stack Operations ( 64 / 64 )
      FF_TTLL -- F = from, T = to, L = len
      11_11XX -- Drop
           DD -- D = id

     0XX_XXXX -- Memory and IO ( 33 / 64)

      1X_XXXX -- Memory ( 16 / 32 )
       1_XXXX -- Memory ( 16 / 16 )
         DDLL -- D = id, L = Len
       0_XXXX -- Unused ( 0 / 16 )

      0X_XXXX -- IO ( 17 / 32 )
       1_XXXX -- Devices ( 12 / 16 ) [3/4 * 4/4]
         DDLL -- D = id, L = len
       0_XXXX -- DMA ( 5 / 16 ) [1/16 + 1/16 + 3/16]
         DDLL -- D = id, L = len
*/
use super::len::*;

pub enum Ins {
    NoOperation,

    // Stack Operations
    DuplicateData { len: Len64 },
    CopyDataToSwap { len: Len64 },
    CopyDataToReturn { len: Len64 },
    CopyDataToHold { len: Len64 },
    CopySwapToData { len: Len64 },
    DuplicateSwap { len: Len64 },
    CopySwapToReturn { len: Len64 },
    CopySwapToHold { len: Len64 },
    CopyReturnToData { len: Len64 },
    CopyReturnToSwap { len: Len64 },
    DuplicateReturn { len: Len64 },
    CopyReturnToHold { len: Len64 },
    CopyHoldToData { len: Len64 },
    CopyHoldToSwap { len: Len64 },
    CopyHoldToReturn { len: Len64 },
    DropData,
    DropSwap,
    DropReturn,

    // Memory
    Literal { len: Len64 },
    Address { len: Len64 },
    Store { len: Len64 },
    Load { len: Len64 },

    // DMA
    DMARead,
    DMAWrite { len: Len32 },
    DMAPoll,

    // Devices
    DeviceRead { len: Len64 },
    DeviceWrite { len: Len64 },
    DevicePoll { len: Len64 },

    // Branching
    Jump { con: bool, rel: bool, len: Len16 },
    Call { len: Len16 },
    Return { len: Len16 },

    // Int Operations
    Add { len: Len64 },
    Subtract { len: Len64 },
    Multiply { len: Len64 },
    Divide { len: Len64 },
    Greater { len: Len64 },
    Less { len: Len64 },
    Equal { len: Len64 },
    NotEqual { len: Len64 },

    // Bitwise Operations
    And { len: Len64 },
    Or { len: Len64 },
    Xor { len: Len64 },
    Not { len: Len64 },
    ShiftL { len: Len64 },
    ShiftR { len: Len64 },

    // Float Operations
    AddF { len: LenF },
    SubtractF { len: LenF },
    MultiplyF { len: LenF },
    DivideF { len: LenF },
    GreaterF { len: LenF },
    LessF { len: LenF },
}

impl From<u8> for Ins {
    fn from(byte: u8) -> Self {
        match byte {
            0b00000000 => Ins::NoOperation,

            0b00000001 => Ins::NoOperation,

            0b0000001_0 => Ins::NoOperation,
            0b0000001_1 => Ins::NoOperation,

            // Call/Return  -- 000001_IL (instruction, length)
            0b000001_00 => Ins::Call { len: Len16::L8 },
            0b000001_01 => Ins::Call { len: Len16::L16 },
            0b000001_10 => Ins::Return { len: Len16::L16 },
            0b000001_11 => Ins::Return { len: Len16::L16 },

            // Jump         -- 00001_CRL (conditional, relative, length)
            0b00001_000 => Ins::Jump {
                con: false,
                rel: false,
                len: Len16::L8,
            },
            0b00001_001 => Ins::Jump {
                con: false,
                rel: false,
                len: Len16::L16,
            },
            0b00001_010 => Ins::Jump {
                con: false,
                rel: true,
                len: Len16::L8,
            },
            0b00001_011 => Ins::Jump {
                con: false,
                rel: true,
                len: Len16::L16,
            },
            0b00001_100 => Ins::Jump {
                con: true,
                rel: false,
                len: Len16::L8,
            },
            0b00001_101 => Ins::Jump {
                con: true,
                rel: false,
                len: Len16::L16,
            },
            0b00001_110 => Ins::Jump {
                con: true,
                rel: true,
                len: Len16::L8,
            },
            0b00001_111 => Ins::Jump {
                con: true,
                rel: true,
                len: Len16::L16,
            },

            // Float Ops    -- 0001_IIIL (instruction, length)
            0b0001_0000 => Ins::AddF { len: LenF::L32 },
            0b0001_0001 => Ins::AddF { len: LenF::L64 },
            0b0001_0010 => Ins::SubtractF { len: LenF::L32 },
            0b0001_0011 => Ins::SubtractF { len: LenF::L64 },
            0b0001_0100 => Ins::MultiplyF { len: LenF::L32 },
            0b0001_0101 => Ins::MultiplyF { len: LenF::L64 },
            0b0001_0110 => Ins::DivideF { len: LenF::L32 },
            0b0001_0111 => Ins::DivideF { len: LenF::L64 },
            0b0001_1000 => Ins::GreaterF { len: LenF::L32 },
            0b0001_1001 => Ins::GreaterF { len: LenF::L64 },
            0b0001_1010 => Ins::LessF { len: LenF::L32 },
            0b0001_1011 => Ins::LessF { len: LenF::L64 },
            0b0001_1100 => Ins::NoOperation,
            0b0001_1101 => Ins::NoOperation,
            0b0001_1110 => Ins::NoOperation,
            0b0001_1111 => Ins::NoOperation,

            // Unused       -- 001_XXXXX
            0b001_00000 => Ins::NoOperation,
            0b001_00001 => Ins::NoOperation,
            0b001_00010 => Ins::NoOperation,
            0b001_00011 => Ins::NoOperation,
            0b001_00100 => Ins::NoOperation,
            0b001_00101 => Ins::NoOperation,
            0b001_00110 => Ins::NoOperation,
            0b001_00111 => Ins::NoOperation,
            0b001_01000 => Ins::NoOperation,
            0b001_01001 => Ins::NoOperation,
            0b001_01010 => Ins::NoOperation,
            0b001_01011 => Ins::NoOperation,
            0b001_01100 => Ins::NoOperation,
            0b001_01101 => Ins::NoOperation,
            0b001_01110 => Ins::NoOperation,
            0b001_01111 => Ins::NoOperation,
            0b001_10000 => Ins::NoOperation,
            0b001_10001 => Ins::NoOperation,
            0b001_10010 => Ins::NoOperation,
            0b001_10011 => Ins::NoOperation,
            0b001_10100 => Ins::NoOperation,
            0b001_10101 => Ins::NoOperation,
            0b001_10110 => Ins::NoOperation,
            0b001_10111 => Ins::NoOperation,
            0b001_11000 => Ins::NoOperation,
            0b001_11001 => Ins::NoOperation,
            0b001_11010 => Ins::NoOperation,
            0b001_11011 => Ins::NoOperation,
            0b001_11100 => Ins::NoOperation,
            0b001_11101 => Ins::NoOperation,
            0b001_11110 => Ins::NoOperation,
            0b001_11111 => Ins::NoOperation,

            // Byte Ops     -- 01_XXXXXX

            // Bitwise Ops  -- 010_IIILL (instruction, length)
            0b010_00000 => Ins::And { len: Len64::L08 },
            0b010_00001 => Ins::And { len: Len64::L16 },
            0b010_00010 => Ins::And { len: Len64::L32 },
            0b010_00011 => Ins::And { len: Len64::L64 },
            0b010_00100 => Ins::Or { len: Len64::L08 },
            0b010_00101 => Ins::Or { len: Len64::L16 },
            0b010_00110 => Ins::Or { len: Len64::L32 },
            0b010_00111 => Ins::Or { len: Len64::L64 },
            0b010_01000 => Ins::Xor { len: Len64::L08 },
            0b010_01001 => Ins::Xor { len: Len64::L16 },
            0b010_01010 => Ins::Xor { len: Len64::L32 },
            0b010_01011 => Ins::Xor { len: Len64::L64 },
            0b010_01100 => Ins::Not { len: Len64::L08 },
            0b010_01101 => Ins::Not { len: Len64::L16 },
            0b010_01110 => Ins::Not { len: Len64::L32 },
            0b010_01111 => Ins::Not { len: Len64::L64 },
            0b010_10000 => Ins::ShiftL { len: Len64::L08 },
            0b010_10001 => Ins::ShiftL { len: Len64::L16 },
            0b010_10010 => Ins::ShiftL { len: Len64::L32 },
            0b010_10011 => Ins::ShiftL { len: Len64::L64 },
            0b010_10100 => Ins::ShiftR { len: Len64::L08 },
            0b010_10101 => Ins::ShiftR { len: Len64::L16 },
            0b010_10110 => Ins::ShiftR { len: Len64::L32 },
            0b010_10111 => Ins::ShiftR { len: Len64::L64 },
            0b010_11000 => Ins::NoOperation,
            0b010_11001 => Ins::NoOperation,
            0b010_11010 => Ins::NoOperation,
            0b010_11011 => Ins::NoOperation,
            0b010_11100 => Ins::NoOperation,
            0b010_11101 => Ins::NoOperation,
            0b010_11110 => Ins::NoOperation,
            0b010_11111 => Ins::NoOperation,

            // Int Math     -- 011_IIILL (instruction, length)
            0b011_00000 => Ins::Add { len: Len64::L08 },
            0b011_00001 => Ins::Add { len: Len64::L16 },
            0b011_00010 => Ins::Add { len: Len64::L32 },
            0b011_00011 => Ins::Add { len: Len64::L64 },
            0b011_00100 => Ins::Subtract { len: Len64::L08 },
            0b011_00101 => Ins::Subtract { len: Len64::L16 },
            0b011_00110 => Ins::Subtract { len: Len64::L32 },
            0b011_00111 => Ins::Subtract { len: Len64::L64 },
            0b011_01000 => Ins::Multiply { len: Len64::L08 },
            0b011_01001 => Ins::Multiply { len: Len64::L16 },
            0b011_01010 => Ins::Multiply { len: Len64::L32 },
            0b011_01011 => Ins::Multiply { len: Len64::L64 },
            0b011_01100 => Ins::Divide { len: Len64::L08 },
            0b011_01101 => Ins::Divide { len: Len64::L16 },
            0b011_01110 => Ins::Divide { len: Len64::L32 },
            0b011_01111 => Ins::Divide { len: Len64::L64 },
            0b011_10000 => Ins::Greater { len: Len64::L08 },
            0b011_10001 => Ins::Greater { len: Len64::L16 },
            0b011_10010 => Ins::Greater { len: Len64::L32 },
            0b011_10011 => Ins::Greater { len: Len64::L64 },
            0b011_10100 => Ins::Less { len: Len64::L08 },
            0b011_10101 => Ins::Less { len: Len64::L16 },
            0b011_10110 => Ins::Less { len: Len64::L32 },
            0b011_10111 => Ins::Less { len: Len64::L64 },
            0b011_11000 => Ins::Equal { len: Len64::L08 },
            0b011_11001 => Ins::Equal { len: Len64::L16 },
            0b011_11010 => Ins::Equal { len: Len64::L32 },
            0b011_11011 => Ins::Equal { len: Len64::L64 },
            0b011_11100 => Ins::NotEqual { len: Len64::L08 },
            0b011_11101 => Ins::NotEqual { len: Len64::L16 },
            0b011_11110 => Ins::NotEqual { len: Len64::L32 },
            0b011_11111 => Ins::NotEqual { len: Len64::L64 },

            // Data Ops     -- 1xxx_xxxx

            // Memory + IO  -- 10xx_xxxx

            // IO           -- 100x_xxxx

            // DMA          -- 1000_xxxx
            0b1000_0000 => Ins::DMARead,
            0b1000_0001 => Ins::NoOperation,
            0b1000_0010 => Ins::NoOperation,
            0b1000_0011 => Ins::NoOperation,
            0b1000_0100 => Ins::DMAWrite { len: Len32::L08 },
            0b1000_0101 => Ins::DMAWrite { len: Len32::L16 },
            0b1000_0110 => Ins::DMAWrite { len: Len32::L32 },
            0b1000_0111 => Ins::NoOperation,
            0b1000_1000 => Ins::DMAPoll,
            0b1000_1001 => Ins::NoOperation,
            0b1000_1010 => Ins::NoOperation,
            0b1000_1011 => Ins::NoOperation,
            0b1000_1100 => Ins::NoOperation,
            0b1000_1101 => Ins::NoOperation,
            0b1000_1110 => Ins::NoOperation,
            0b1000_1111 => Ins::NoOperation,

            // Devices      -- 1001_xxxx
            0b1001_0000 => Ins::DeviceRead { len: Len64::L08 },
            0b1001_0001 => Ins::DeviceRead { len: Len64::L16 },
            0b1001_0010 => Ins::DeviceRead { len: Len64::L32 },
            0b1001_0011 => Ins::DeviceRead { len: Len64::L64 },
            0b1001_0100 => Ins::DeviceWrite { len: Len64::L08 },
            0b1001_0101 => Ins::DeviceWrite { len: Len64::L16 },
            0b1001_0110 => Ins::DeviceWrite { len: Len64::L32 },
            0b1001_0111 => Ins::DeviceWrite { len: Len64::L64 },
            0b1001_1000 => Ins::DevicePoll { len: Len64::L08 },
            0b1001_1001 => Ins::DevicePoll { len: Len64::L16 },
            0b1001_1010 => Ins::DevicePoll { len: Len64::L32 },
            0b1001_1011 => Ins::DevicePoll { len: Len64::L64 },
            0b1001_1100 => Ins::NoOperation,
            0b1001_1101 => Ins::NoOperation,
            0b1001_1110 => Ins::NoOperation,
            0b1001_1111 => Ins::NoOperation,

            // Memory       -- 101x_xxxx

            // unused       -- 1010_xxxx
            0b1010_0000 => Ins::NoOperation,
            0b1010_0001 => Ins::NoOperation,
            0b1010_0010 => Ins::NoOperation,
            0b1010_0011 => Ins::NoOperation,
            0b1010_0100 => Ins::NoOperation,
            0b1010_0101 => Ins::NoOperation,
            0b1010_0110 => Ins::NoOperation,
            0b1010_0111 => Ins::NoOperation,
            0b1010_1000 => Ins::NoOperation,
            0b1010_1001 => Ins::NoOperation,
            0b1010_1010 => Ins::NoOperation,
            0b1010_1011 => Ins::NoOperation,
            0b1010_1100 => Ins::NoOperation,
            0b1010_1101 => Ins::NoOperation,
            0b1010_1110 => Ins::NoOperation,
            0b1010_1111 => Ins::NoOperation,

            // Memory       -- 1011_xxxx
            0b1011_0000 => Ins::Literal { len: Len64::L08 },
            0b1011_0001 => Ins::Literal { len: Len64::L16 },
            0b1011_0010 => Ins::Literal { len: Len64::L32 },
            0b1011_0011 => Ins::Literal { len: Len64::L64 },
            0b1011_0100 => Ins::Address { len: Len64::L08 },
            0b1011_0101 => Ins::Address { len: Len64::L16 },
            0b1011_0110 => Ins::Address { len: Len64::L32 },
            0b1011_0111 => Ins::Address { len: Len64::L64 },
            0b1011_1000 => Ins::Store { len: Len64::L08 },
            0b1011_1001 => Ins::Store { len: Len64::L16 },
            0b1011_1010 => Ins::Store { len: Len64::L32 },
            0b1011_1011 => Ins::Store { len: Len64::L64 },
            0b1011_1100 => Ins::Load { len: Len64::L08 },
            0b1011_1101 => Ins::Load { len: Len64::L16 },
            0b1011_1110 => Ins::Load { len: Len64::L32 },
            0b1011_1111 => Ins::Load { len: Len64::L64 },

            // Stack Ops    -- 11xx_xxxx
            // Data Stack   -- 1100_xxxx
            0b1100_0000 => Ins::DuplicateData { len: Len64::L08 },
            0b1100_0001 => Ins::DuplicateData { len: Len64::L16 },
            0b1100_0010 => Ins::DuplicateData { len: Len64::L32 },
            0b1100_0011 => Ins::DuplicateData { len: Len64::L64 },
            0b1100_0100 => Ins::CopyDataToSwap { len: Len64::L08 },
            0b1100_0101 => Ins::CopyDataToSwap { len: Len64::L16 },
            0b1100_0110 => Ins::CopyDataToSwap { len: Len64::L32 },
            0b1100_0111 => Ins::CopyDataToSwap { len: Len64::L64 },
            0b1100_1000 => Ins::CopyDataToReturn { len: Len64::L08 },
            0b1100_1001 => Ins::CopyDataToReturn { len: Len64::L16 },
            0b1100_1010 => Ins::CopyDataToReturn { len: Len64::L32 },
            0b1100_1011 => Ins::CopyDataToReturn { len: Len64::L64 },
            0b1100_1100 => Ins::CopyDataToHold { len: Len64::L08 },
            0b1100_1101 => Ins::CopyDataToHold { len: Len64::L16 },
            0b1100_1110 => Ins::CopyDataToHold { len: Len64::L32 },
            0b1100_1111 => Ins::CopyDataToHold { len: Len64::L64 },

            // Swap Stack   -- 1101_xxxx
            0b1101_0000 => Ins::CopySwapToData { len: Len64::L08 },
            0b1101_0001 => Ins::CopySwapToData { len: Len64::L16 },
            0b1101_0010 => Ins::CopySwapToData { len: Len64::L32 },
            0b1101_0011 => Ins::CopySwapToData { len: Len64::L64 },
            0b1101_0100 => Ins::DuplicateSwap { len: Len64::L08 },
            0b1101_0101 => Ins::DuplicateSwap { len: Len64::L16 },
            0b1101_0110 => Ins::DuplicateSwap { len: Len64::L32 },
            0b1101_0111 => Ins::DuplicateSwap { len: Len64::L64 },
            0b1101_1000 => Ins::CopySwapToReturn { len: Len64::L08 },
            0b1101_1001 => Ins::CopySwapToReturn { len: Len64::L16 },
            0b1101_1010 => Ins::CopySwapToReturn { len: Len64::L32 },
            0b1101_1011 => Ins::CopySwapToReturn { len: Len64::L64 },
            0b1101_1100 => Ins::CopySwapToHold { len: Len64::L08 },
            0b1101_1101 => Ins::CopySwapToHold { len: Len64::L16 },
            0b1101_1110 => Ins::CopySwapToHold { len: Len64::L32 },
            0b1101_1111 => Ins::CopySwapToHold { len: Len64::L64 },

            // Return Stack   -- 1110_xxxx
            0b1110_0000 => Ins::CopyReturnToData { len: Len64::L08 },
            0b1110_0001 => Ins::CopyReturnToData { len: Len64::L16 },
            0b1110_0010 => Ins::CopyReturnToData { len: Len64::L32 },
            0b1110_0011 => Ins::CopyReturnToData { len: Len64::L64 },
            0b1110_0100 => Ins::CopyReturnToSwap { len: Len64::L08 },
            0b1110_0101 => Ins::CopyReturnToSwap { len: Len64::L16 },
            0b1110_0110 => Ins::CopyReturnToSwap { len: Len64::L32 },
            0b1110_0111 => Ins::CopyReturnToSwap { len: Len64::L64 },
            0b1110_1000 => Ins::DuplicateReturn { len: Len64::L08 },
            0b1110_1001 => Ins::DuplicateReturn { len: Len64::L16 },
            0b1110_1010 => Ins::DuplicateReturn { len: Len64::L32 },
            0b1110_1011 => Ins::DuplicateReturn { len: Len64::L64 },
            0b1110_1100 => Ins::CopyReturnToHold { len: Len64::L08 },
            0b1110_1101 => Ins::CopyReturnToHold { len: Len64::L16 },
            0b1110_1110 => Ins::CopyReturnToHold { len: Len64::L32 },
            0b1110_1111 => Ins::CopyReturnToHold { len: Len64::L64 },

            // Hold and Drop    -- 1111_xxxx
            0b1111_0000 => Ins::CopyHoldToData { len: Len64::L08 },
            0b1111_0001 => Ins::CopyHoldToData { len: Len64::L16 },
            0b1111_0010 => Ins::CopyHoldToData { len: Len64::L32 },
            0b1111_0011 => Ins::CopyHoldToData { len: Len64::L64 },
            0b1111_0100 => Ins::CopyHoldToSwap { len: Len64::L08 },
            0b1111_0101 => Ins::CopyHoldToSwap { len: Len64::L16 },
            0b1111_0110 => Ins::CopyHoldToSwap { len: Len64::L32 },
            0b1111_0111 => Ins::CopyHoldToSwap { len: Len64::L64 },
            0b1111_1000 => Ins::CopyHoldToReturn { len: Len64::L08 },
            0b1111_1001 => Ins::CopyHoldToReturn { len: Len64::L16 },
            0b1111_1010 => Ins::CopyHoldToReturn { len: Len64::L32 },
            0b1111_1011 => Ins::CopyHoldToReturn { len: Len64::L64 },
            0b1111_1100 => Ins::DropData,
            0b1111_1101 => Ins::DropSwap,
            0b1111_1110 => Ins::DropReturn,
            0b1111_1111 => Ins::NoOperation,
        }
    }
}

impl std::fmt::Display for Ins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Ins::NoOperation => "NOP".into(),
            Ins::DuplicateData { len } => format!("DUP{} DATA", len),
            Ins::CopyDataToSwap { len } => format!("COPY{} DATA SWAP", len),
            Ins::CopyDataToReturn { len } => format!("COPY{} DATA RTRN", len),
            Ins::CopyDataToHold { len } => format!("COPY{} DATA HOLD", len),
            Ins::CopySwapToData { len } => format!("COPY{} SWAP DATA", len),
            Ins::DuplicateSwap { len } => format!("DUP{} SWAP", len),
            Ins::CopySwapToReturn { len } => format!("COPY{} SWAP RTRN", len),
            Ins::CopySwapToHold { len } => format!("COPY{} SWAP HOLD", len),
            Ins::CopyReturnToData { len } => format!("COPY{} RTRN DATA", len),
            Ins::CopyReturnToSwap { len } => format!("COPY{} RTRN SWAP", len),
            Ins::DuplicateReturn { len } => format!("DUP{} RTRN", len),
            Ins::CopyReturnToHold { len } => format!("COPY{} RTRN HOLD", len),
            Ins::CopyHoldToData { len } => format!("COPY{} HOLD DATA", len),
            Ins::CopyHoldToSwap { len } => format!("COPY{} HOLD SWAP", len),
            Ins::CopyHoldToReturn { len } => format!("COPY{} HOLD RTRN", len),
            Ins::DropData => format!("DROP DATA"),
            Ins::DropSwap => format!("DROP SWAP"),
            Ins::DropReturn => format!("DROP RTRN"),
            Ins::Jump {
                len,
                con: conditional,
                rel: relative,
            } => match conditional {
                true => match relative {
                    true => format!("JUMP{} REL COND", len),
                    false => format!("JUMP{} COND", len),
                },
                false => match relative {
                    true => format!("JUMP{} REL", len),
                    false => format!("JUMP{}", len),
                },
            },
            Ins::Call { len } => format!("CALL{}", len),
            Ins::Return { len } => format!("RTRN{}", len),
            Ins::Address { len } => format!("SET ADDR{}", len),
            Ins::Store { len } => format!("MEM STOR{}", len),
            Ins::Load { len } => format!("MEM LOAD{}", len),
            Ins::Literal { len } => format!("LIT{}", len),
            Ins::DMARead => format!("DMA READ"),
            Ins::DMAWrite { len } => format!("DMA WRIT{}", len),
            Ins::DMAPoll => format!("DMA POLL"),
            Ins::DeviceRead { len } => format!("DEV READ{}", len),
            Ins::DeviceWrite { len } => format!("DEV WRIT{}", len),
            Ins::DevicePoll { len } => format!("DEV POLL{}", len),
            Ins::Add { len } => format!("+{}", len),
            Ins::Subtract { len } => format!("-{}", len),
            Ins::Multiply { len } => format!("*{}", len),
            Ins::Divide { len } => format!("/{}", len),
            Ins::Greater { len } => format!(">{}", len),
            Ins::Less { len } => format!("<{}", len),
            Ins::Equal { len } => format!("=={}", len),
            Ins::NotEqual { len } => format!("!={}", len),
            Ins::AddF { len } => format!("+F{}", len),
            Ins::SubtractF { len } => format!("-F{}", len),
            Ins::MultiplyF { len } => format!("*F{}", len),
            Ins::DivideF { len } => format!("/F{}", len),
            Ins::GreaterF { len } => format!(">F{}", len),
            Ins::LessF { len } => format!("<F{}", len),
            Ins::And { len } => format!("&{}", len),
            Ins::Or { len } => format!("|{}", len),
            Ins::Xor { len } => format!("^{}", len),
            Ins::Not { len } => format!("!{}", len),
            Ins::ShiftL { len } => format!("<<{}", len),
            Ins::ShiftR { len } => format!(">>{}", len),
        };
        write!(f, "{}", s)
    }
}
