mod bytecoil;
mod context;
mod import;
mod library;
mod macros;
mod module;
mod routines;

pub use bytecoil::ByteCoIL;
pub use context::Context;
pub use import::Import;
pub use library::Library;
pub use macros::Macro;
pub use module::Module;
pub use routines::Routine;

type Hash = [u8; 32];
type ByteCo = Vec<u8>;
type NameTable = std::collections::HashMap<String, Hash>;
