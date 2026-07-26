mod bus;
mod cartridge;
mod cpu;
pub mod display;
mod input;
mod memory;
mod ppu;

pub struct Emulator {
    pub cpu: cpu::CPU,
    pub bus: bus::Bus,
    pub display: display::Display,
}

impl Emulator {
    pub fn new() -> Emulator {
        Self {
            cpu: cpu::CPU::new(),
            bus: bus::Bus::new(),
            display: display::Display::new(),
        }
    }
}
