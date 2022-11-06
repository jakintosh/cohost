pub(crate) mod core;

fn main() {
    let mut cpu = core::CPU::new();
    cpu.run();
}
