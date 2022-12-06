use super::ByteCo;
use crate::assembler::tokens::Label;

#[derive(Clone)]
pub enum ByteCoIL {
    Assembled(ByteCo),
    Comment(String),
    RoutineDef(String),
    RoutineCallLocal(Label),
    RoutineCallExported(Label),
    RoutineAddressLocal(Label),
    RoutineAddressExported(Label),
    RoutineEnd,
    AnchorDef(Label),
    AnchorRel(Label),
    AnchorAbs(Label),
}
impl ByteCoIL {
    pub fn len(&self) -> usize {
        match self {
            Self::Assembled(byteco) => byteco.len(),
            Self::Comment(string) => string.len() + 1,
            Self::RoutineDef(..) => 0,
            Self::RoutineCallLocal(_) => 4,
            Self::RoutineCallExported(_) => 33,
            Self::RoutineAddressLocal(_) => 3,
            Self::RoutineAddressExported(_) => 33,
            Self::RoutineEnd => 1,
            Self::AnchorDef(..) => 0,
            Self::AnchorRel(_) => 2,
            Self::AnchorAbs(_) => 3,
        }
    }
}
impl std::fmt::Display for ByteCoIL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assembled(byteco) => write!(f, "Assembled({:?})", byteco),
            Self::Comment(string) => write!(f, "Comment({})", string),
            Self::RoutineDef(name) => write!(f, "RoutineDef({})", name),
            Self::RoutineCallLocal(label) => write!(f, "RoutineCallLocal({:?})", label),
            Self::RoutineCallExported(name) => write!(f, "RoutineCallExported({:?})", name),
            Self::RoutineAddressLocal(name) => write!(f, "RoutineAddressLocal({:?})", name),
            Self::RoutineAddressExported(name) => write!(f, "RoutineAddressExported({:?})", name),
            Self::RoutineEnd => write!(f, "RoutineEnd"),
            Self::AnchorDef(name) => write!(f, "AnchorDef({:?})", name),
            Self::AnchorRel(name) => write!(f, "AnchorRel({:?})", name),
            Self::AnchorAbs(name) => write!(f, "AnchorAbs({:?})", name),
        }
    }
}
