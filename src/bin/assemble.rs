use cohost::assembler::{
    parsing::parse_text,
    representation::{Context, Library, Module},
};

use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

const HELP: &str = "
assemble v1 by @jakintosh

assembles machine instructions for the coalescent core virtual CPU.
pass in the location of the '.co' assembly file, and a location for
the assembled binary output.

USAGE:
`-s` or `--source`              | file with source code
`-o` or `--output` (required)   | file for compiled output

VALID ARGUMENT SYNTAX:
    `-s=file`
    `-s file`
    `--source=file`
    `--source file`";

struct Parameters {
    source: PathBuf,
    output: PathBuf,
}
impl TryFrom<std::env::Args> for Parameters {
    type Error = String;

    fn try_from(mut args: std::env::Args) -> Result<Self, Self::Error> {
        fn parse_arg(args: &mut std::env::Args, token: String) -> Option<(String, String)> {
            match token.split('=').collect::<Vec<_>>() {
                subtokens if subtokens.len() == 2 => {
                    Some((subtokens[0].into(), subtokens[1].into()))
                }
                _ => Some((token, args.next()?)),
            }
        }
        fn map_arg(
            map: &HashMap<String, String>,
            short: &str,
            long: &str,
            default: Result<String, String>,
        ) -> Result<String, String> {
            if map.contains_key(short) {
                Ok(map[short].clone())
            } else if map.contains_key(long) {
                Ok(map[long].clone())
            } else {
                default
            }
        }

        args.next(); // skip first arg, bin location

        let mut map: HashMap<String, String> = HashMap::new();
        while let Some(arg) = args.next() {
            let token = {
                if let Some(t) = arg.strip_prefix("--") {
                    String::from(t)
                } else if let Some(t) = arg.strip_prefix("-") {
                    String::from(t)
                } else {
                    arg
                }
            };

            if let Some((key, value)) = parse_arg(&mut args, token) {
                map.insert(key, value);
            }
        }

        let source = map_arg(&map, "s", "source", Err("--source param missing".into()))?.into();
        let output = map_arg(&map, "o", "output", Err("--output param missing".into()))?.into();
        Ok::<Parameters, String>(Parameters { source, output })
    }
}

fn main() -> Result<(), String> {
    let Parameters { source, output } = std::env::args().try_into().map_err(|e| {
        println!("{}", HELP);
        format!("{}", e)
    })?;

    let assembly_text = match read_to_string(source) {
        Ok(asm) => asm,
        Err(e) => return Err(format!("Couldn't read source: {}", e)),
    };
    let text_tokens = parse_text(&assembly_text)?;
    let module = Module::from_text_tokens(text_tokens)?;
    println!("Module:\n\n{}", module);
    // let library = Library::new();
    // let context = Context::new(&library, module)?;
    // let byteco = context.assemble()?;

    Ok(())

    // let binary = binary_from_text(&assembly);

    // std::fs::write(output, binary).map_err(|e| format!("{}", e))
}
