use std::{collections::HashMap, path::PathBuf};

const HELP: &str = "
assemble v1 by @jakintosh

USAGE:
`-s` or `--source` (optional, default is '.')  | file with source code
`-o` or `--output` (required)                  | file for compiled output

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

        let source = map_arg(&map, "s", "source", Ok(".".into()))?.into();
        let output = map_arg(&map, "o", "output", Err("--output param missing".into()))?.into();
        Ok::<Parameters, String>(Parameters { source, output })
    }
}

fn main() -> Result<(), String> {
    let Parameters { source, output } = std::env::args().try_into().map_err(|e| {
        println!("{}", HELP);
        format!("{}", e)
    })?;

    let assembly = std::fs::read_to_string(source).expect("couldn't read source");
    let tokens: Vec<_> = assembly.split_whitespace().collect();
    let mut binary: Vec<u8> = Vec::with_capacity(tokens.len());

    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i];
        println!("reading token: {}", token);
        let Some(opcode) = cohost::core::str_to_opcode(token) else {
            panic!("received invalid token {}", token);
        };
        binary.push(opcode);

        // uh, check for literals i guess?
        if opcode >= 176 && opcode < 180 {
            i += 1;
            let lit_token = tokens[i];
            println!("reading token: {}", lit_token);
            match opcode {
                176 => {
                    let Some(lit) = parse_lit8(lit_token) else {
                        panic!("couldn't parse lit: {}", token)
                    };
                    binary.push(lit);
                }
                177 => {
                    let Some(lit) = parse_lit16(lit_token) else {
                        panic!("couldn't parse lit: {}", token)
                    };
                    binary.extend_from_slice(&lit.to_le_bytes());
                }
                178 => {
                    let Some(lit) = parse_lit32(lit_token) else {
                        panic!("couldn't parse lit: {}", token)
                    };
                    binary.extend_from_slice(&lit.to_le_bytes());
                }
                179 => {
                    let Some(lit) = parse_lit64(lit_token) else {
                        panic!("couldn't parse lit: {}", token)
                    };
                    binary.extend_from_slice(&lit.to_le_bytes());
                }
                _ => {} // do nothing
            }
        }

        i += 1;
    }

    std::fs::write(output, binary).map_err(|e| format!("{}", e))
}

fn parse_lit8(token: &str) -> Option<u8> {
    let (token, radix) = match token.strip_prefix("0x") {
        Some(hex) => (hex, 16),
        None => (token, 10),
    };
    match u8::from_str_radix(token, radix) {
        Ok(lit) => Some(lit),
        Err(_) => None,
    }
}
fn parse_lit16(token: &str) -> Option<u16> {
    let (token, radix) = match token.strip_prefix("0x") {
        Some(hex) => (hex, 16),
        None => (token, 10),
    };
    match u16::from_str_radix(token, radix) {
        Ok(lit) => Some(lit),
        Err(_) => None,
    }
}
fn parse_lit32(token: &str) -> Option<u32> {
    let (token, radix) = match token.strip_prefix("0x") {
        Some(hex) => (hex, 16),
        None => (token, 10),
    };
    match u32::from_str_radix(token, radix) {
        Ok(lit) => Some(lit),
        Err(_) => None,
    }
}
fn parse_lit64(token: &str) -> Option<u64> {
    let (token, radix) = match token.strip_prefix("0x") {
        Some(hex) => (hex, 16),
        None => (token, 10),
    };
    match u64::from_str_radix(token, radix) {
        Ok(lit) => Some(lit),
        Err(_) => None,
    }
}
