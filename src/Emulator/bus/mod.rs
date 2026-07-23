use crate::emulator::cartridge;
use crate::emulator::memory;
use crate::emulator::ppu;

pub struct Bus {
    ram: memory::Memory,
    cartridge: cartridge::Cartridge,
    // Blargg's tests use this region for state and text streams
    wram: [u8; 0x2000],
    pub ppu: ppu::PPU,

    pub nmi: bool,
}

impl Bus {
    pub fn new() -> Self {
        println!("Initialized Bus");
        let memory = memory::Memory::new();
        let cart = cartridge::Cartridge::new();
        Self {
            ram: memory,
            cartridge: cart,
            wram: [0; 0x2000],
            ppu: ppu::PPU::new(),
            nmi: false,
        }
    }

    pub fn step_ppu(&mut self, cycles: u32) {
        for _ in 0..(cycles * 3) {
            self.ppu.step(&self.cartridge);
            if self.ppu.request_nmi
            {
                self.nmi = true;
                self.ppu.request_nmi = false;
            }
        }
    }

    pub fn poll_nmi(&mut self) -> bool {
        if self.nmi {
            self.nmi = false; // Acknowledge signal
            true
        } else {
            false
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram.read(address),

            // PPU Register range (0x2000 - 0x3FFF, mirrored every 8 bytes)
            0x2000..=0x3FFF => self.ppu.cpu_read(&self.cartridge, address),

            0x6000..=0x7FFF => self.wram[(address - 0x6000) as usize],

            0x8000..=0xFFFF => {
                let rom_address = address - 0x8000;
                let mirrored_address = rom_address as usize % self.cartridge.prg_rom.len();
                self.cartridge.prg_rom[mirrored_address]
            }
            _ => 0,
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram.write(address, data);
            }

            // PPU Register range (0x2000 - 0x3FFF, mirrored every 8 bytes)
            0x2000..=0x3FFF => {
                self.ppu.cpu_write(&mut self.cartridge, address, data);
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
        self.cartridge.prg_rom = rom_data.to_vec();
    }
}
