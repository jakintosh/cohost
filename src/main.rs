use device::{Device, Devices};
use std::collections::HashMap;

mod core;
mod device;

fn main() {
    // init CPU
    let mut cpu = core::CPU::new();

    // initialize all devices
    let console = Box::new(device::Console::new());

    // register all devices
    let mut devices: HashMap<Devices, Box<dyn Device>> = HashMap::new();
    devices.insert(Devices::Console, console);

    // connect devices to CPU
    for device_type in devices.keys() {
        let id = device_type.clone().into();
        cpu.connect_device(id);
    }

    // run CPU
    loop {
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
            eprint!("unregistered device");
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
