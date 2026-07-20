use crate::emulator::memory;

pub struct Bus {
    ram: memory::Memory,
    prg_rom: Vec<u8>,
    
    // Blargg's tests use this region for state and text streams
    wram: [u8; 0x2000],
}

impl Bus {
    pub fn new() -> Self {
        println!("Initialized Bus");
        let memory = memory::Memory::new();
        Self {
            ram: memory,
            prg_rom: vec![0; 0x8000],
            wram: [0; 0x2000],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram.read(address),
            0x2002 => {
                0x80 // simulate existace of PPU
            }
            0x6000..=0x7FFF => self.wram[(address - 0x6000) as usize],

            0x8000..=0xFFFF => {
                let rom_address = address - 0x8000;
                let mirrored_address = rom_address as usize % self.prg_rom.len();
                self.prg_rom[mirrored_address]
            }
            _ => 0,
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram.write(address, data);
            }

            // Catch CPU writes to Blargg's test window
            0x6000..=0x7FFF => {
                let offset = (address - 0x6000) as usize;
                self.wram[offset] = data;

                if address == 0x6004 {
                    print!("{}", data as char);

                    use std::io::{self, Write};
                    let _ = io::stdout().flush();
                }
            }
            _ => {}
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.prg_rom = rom_data.to_vec();
    }
}
