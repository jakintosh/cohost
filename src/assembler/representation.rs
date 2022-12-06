mod bytecoil;
mod context;
mod library;
mod macros;
mod module;
mod routines;

pub use bytecoil::ByteCoIL;
pub use context::Context;
pub use library::Library;
pub use macros::Macro;
pub use module::Module;
pub use routines::Routine;

type Hash = [u8; 32];
type ByteCo = Vec<u8>;

// pub enum Component {
//     Routine {
//         export: bool,
//         label: String,
//         tokens: Vec<SourceToken>,
//     },
//     Macro {
//         label: String,
//         tokens: Vec<SourceToken>,
//     },
//     Comment {
//         string: String,
//     },
// }

// pub struct Module {
//     pub components: Vec<Component>,
// }

// impl Display for Component {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let toks;
//         match self {
//             Component::Routine {
//                 export,
//                 label,
//                 tokens,
//             } => {
//                 let exported: String = match export {
//                     true => "Exported ".into(),
//                     false => "".into(),
//                 };
//                 write!(f, "{}Routine: \"{}\"", exported, label)?;
//                 toks = tokens;
//             }
//             Component::Macro { label, tokens } => {
//                 write!(f, "Macro: \"{}\"", label)?;
//                 toks = tokens;
//             }
//             Component::Comment { string } => {
//                 return write!(f, "Comment ( {} )", string);
//             }
//         };

//         for token in toks {
//             write!(f, "\n - {}", token)?;
//         }

//         Ok(())
//     }
// }
// impl Display for Module {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut buffer = String::new();
//         for c in &self.components {
//             buffer.push_str(&format!("{}\n\n", c));
//         }

//         write!(f, "{}", buffer.trim_end())
//     }
// }
