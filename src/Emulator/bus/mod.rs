use crate::Emulator::memory;

pub struct Bus {
    ram: memory::memory,
}

impl Bus {
    pub fn new() -> Self {
        let memory = memory::memory::new();
        Self { ram: memory }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..0x1FFF => self.ram.read(address),
            _ => 0,
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..0x1FFF => {
                self.ram.write(address, data);
            }
            _ => {}
        }
    }
}
