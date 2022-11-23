use cohost::core;
use cohost::device::{Device, Devices};
use std::{
    collections::HashMap,
    io::{Read, Write},
    path::PathBuf,
};

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
    rom: PathBuf,
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

        let rom = map_arg(&map, "r", "rom", Ok(".".into()))?.into();
        Ok::<Parameters, String>(Parameters { rom })
    }
}

fn main() -> Result<(), String> {
    let Parameters { rom } = std::env::args().try_into().map_err(|e| {
        println!("{}", HELP);
        format!("{}", e)
    })?;

    let Ok(rom) = std::fs::read(rom) else {
        panic!("couldn't load rom");
    };

    // init CPU
    let mut cpu = core::CPU::new();
    cpu.load_rom(rom);

    // // initialize all devices
    // let console = Box::new(device::Console::new());

    // // register all devices
    let mut devices: HashMap<Devices, Box<dyn Device>> = HashMap::new();
    // devices.insert(Devices::Console, console);

    // // connect devices to CPU
    // for device_type in devices.keys() {
    //     let id = device_type.clone().into();
    //     cpu.connect_device(id);
    // }

    // run CPU
    loop {
        draw(&cpu);
        cpu.execute();
        check_dmas(&mut cpu);
        check_devices(&mut cpu, &mut devices);
    }
}

fn check_dmas(cpu: &mut core::CPU) {
    for dma in cpu.dma_controllers {
        let has_request = core::DMA::REQ_BIT & dma.status_reg != 0;
        if has_request {
            // process request
            let payload_range = dma.address as usize..(dma.address + dma.buffer_len) as usize;
            let payload = &cpu.memory[payload_range];
        }
    }
}

fn check_devices(cpu: &mut core::CPU, devices: &mut HashMap<Devices, Box<dyn Device>>) {
    for mut slot in cpu.devices {
        // get the device for given slot
        let device_type = slot.identifier.into();
        let Some(device) = devices.get_mut(&device_type) else {
            // eprint!("unregistered device");
            continue;
        };

        // check status registers
        let cpu_send = core::DeviceSlot::SEND_FLAG & slot.status_reg != 0;
        let cpu_done = core::DeviceSlot::DONE_FLAG & slot.status_reg != 0;
        let cpu_block = core::DeviceSlot::BLOCK_FLAG & slot.status_reg != 0;

        // if cpu is sending data, receive it
        if cpu_send {
            device.recv(&slot.out_buffer);
            if cpu_done {
                slot.status_reg ^= core::DeviceSlot::SEND_FLAG; // turn off
                slot.status_reg ^= core::DeviceSlot::DONE_FLAG; // turn off
            }
        }

        // don't read from device if cpu is blocking until outgoing is done
        if !(cpu_send && !cpu_done && cpu_block) {
            // if device has data waiting, pass it in
            while let Some(device_buffer) = device.poll() {
                slot.in_buffer.copy_from_slice(&device_buffer);
                cpu.interrupt(slot.vector);
            }
        }
    }
}

fn draw(cpu: &core::CPU) {
    // clear terminal screen
    print!("{}[2J", 27 as char);

    // print out memory and program counter
    println!();
    println!("Memory and PC");
    println!("=============");
    println!();
    const PAGE_SIZE: usize = 16;
    let pc = cpu.program_counter;
    let page_start = (pc as usize / PAGE_SIZE) * PAGE_SIZE;
    let page_end = page_start + PAGE_SIZE;
    let pc_cursor = pc as usize % PAGE_SIZE;
    let page = &cpu.memory[page_start..page_end];
    for (i, byte) in page.iter().enumerate() {
        match i == pc_cursor {
            true => println!(
                "> {:#08X} {:02X}    {}",
                page_start + i,
                byte,
                core::Instruction::from(byte.clone())
            ),
            false => println!("  {:#08X} {:02X}", page_start + i, byte),
        }
    }
    println!();
    println!();

    // print out stacks
    println!("Stacks");
    println!("======");
    println!();
    println!("DATA | LEN({:03}) | {}", cpu.data_st.len(), cpu.data_st);
    println!("SWAP | LEN({:03}) | {}", cpu.swap_st.len(), cpu.swap_st);
    println!("RTRN | LEN({:03}) | {}", cpu.return_st.len(), cpu.return_st);
    println!("HOLD |  8B REG  | {}", cpu.hold_reg);
    println!();
    println!();

    // Read a single byte and discard
    print!("ENTER >>");
    std::io::stdout().flush().unwrap();

    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
    println!();
}
