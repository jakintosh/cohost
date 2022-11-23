#[repr(u8)]
#[derive(Clone, Copy)]
pub enum LenF {
    L32 = 4,
    L64 = 8,
}
impl From<u8> for LenF {
    fn from(byte: u8) -> Self {
        let byte = byte & 0b0000_0001;
        match byte == 0 {
            true => LenF::L32,
            false => LenF::L64,
        }
    }
}
impl std::fmt::Display for LenF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LenF::L32 => "32",
            LenF::L64 => "64",
        };
        write!(f, "{}", s)
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Len16 {
    L8 = 1,
    L16 = 2,
}
impl From<u8> for Len16 {
    fn from(byte: u8) -> Self {
        let byte = byte & 0b0000_0001;
        match byte == 0 {
            true => Len16::L8,
            false => Len16::L16,
        }
    }
}
impl std::fmt::Display for Len16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Len16::L8 => "8",
            Len16::L16 => "16",
        };
        write!(f, "{}", s)
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Len32 {
    L08 = 1,
    L16 = 2,
    L32 = 4,
}
impl From<u8> for Len32 {
    fn from(byte: u8) -> Self {
        let byte = byte & 0b0000_0011;
        match byte {
            0 => Len32::L08,
            1 => Len32::L16,
            2 => Len32::L32,
            3 => panic!("received invalid `11` bit pattern for u8 -> Len32"),
            _ => unreachable!("len32 from u8"),
        }
    }
}
impl std::fmt::Display for Len32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Len32::L08 => "8",
            Len32::L16 => "16",
            Len32::L32 => "32",
        };
        write!(f, "{}", s)
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Len64 {
    L08 = 1,
    L16 = 2,
    L32 = 4,
    L64 = 8,
}
impl From<u8> for Len64 {
    fn from(byte: u8) -> Self {
        let byte = byte & 0b0000_0011;
        match byte {
            0 => Len64::L08,
            1 => Len64::L16,
            2 => Len64::L32,
            3 => Len64::L64,
            _ => unreachable!("len64 from u8"),
        }
    }
}
impl std::fmt::Display for Len64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Len64::L08 => "8",
            Len64::L16 => "16",
            Len64::L32 => "32",
            Len64::L64 => "64",
        };
        write!(f, "{}", s)
    }
}
