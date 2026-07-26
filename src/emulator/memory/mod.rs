pub struct Memory {
    ram: [u8; 2048],
}

impl Memory {
    pub fn new() -> Self {
        println!("Initialized Memory");
        Self { ram: [0; 2048] }
    }

    pub fn read(&self, address: u16) -> u8 {
        // the cpu memory is mirrored 3 times
        match address {
            0x0000..=0x1FFF => {
                let physical_index = (address & 0x07FF) as usize;
                self.ram[physical_index]
            }
            _ => {
                return 0;
            }
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => {
                let physical_index = (address & 0x07FF) as usize;
                self.ram[physical_index] = data;
            }
            _ => {}
        }
    }
}
