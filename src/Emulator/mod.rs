mod bus;
mod cpu;
mod memory;

pub struct Emulator {
    pub cpu: cpu::CPU,
    pub bus: bus::Bus,
}

impl Emulator {
    pub fn new() -> Emulator {
        Self {
            cpu: cpu::CPU::new(),
            bus: bus::Bus::new(),
        }
    }
}
