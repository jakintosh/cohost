pub(crate) mod core;

fn main() {
    let mut cpu = core::CPU::new();
    loop {
        cpu.execute();
        check_dma(&cpu.dma_controller, &cpu.memory);
    }
}

fn check_dma(mut dma: &core::DMA, memory: &[u8; 65_536]) {
    let has_request = core::DMA::REQUEST_BIT & dma.status_reg != 0;
    if has_request {
        // process request
        let payload_range = dma.address as usize..(dma.address + dma.length) as usize;
        let payload = &memory[payload_range];
    }
}
