use crate::emulator::cartridge;
use crate::emulator::input;
use crate::emulator::memory;
use crate::emulator::ppu;

pub struct Bus {
    ram: memory::Memory,
    cartridge: cartridge::Cartridge,
    pub input: input::Controller,
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
            input: input::Controller::new(),
            wram: [0; 0x2000],
            ppu: ppu::PPU::new(),
            nmi: false,
        }
    }

    pub fn step_ppu(&mut self, cycles: u32) {
        for _ in 0..(cycles * 3) {
            self.ppu.step(&mut self.cartridge);
            if self.ppu.request_nmi {
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
            0x2000..=0x3FFF => self.ppu.cpu_read(&mut self.cartridge, address),
            0x4016 => self.input.get_input(),

            0x6000..=0x7FFF => self.wram[(address - 0x6000) as usize],

            0x8000..=0xFFFF => self.cartridge.mapper.as_mut().unwrap().cpu_read(address),
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

            // OAM DMA: copies 256 bytes from 0xXX00-0xXXFF (XX = data) into
            // PPU OAM, starting at the current OAM address.
            0x4014 => {
                self.oam_dma(data);
            }
            0x4016 => self.input.write_strobe(data),

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
            0x8000..=0xFFFF => {
            self.cartridge.mapper.as_mut().unwrap().cpu_write(address, data);
        }
            _ => {}
        }
    }

    /// Performs an OAM DMA transfer triggered by a write to 0x4014
    fn oam_dma(&mut self, page: u8) {
        let start = (page as u16) << 8;
        for i in 0..256u16 {
            let byte = self.read(start + i);
            self.ppu.oam_dma_write(byte);
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.cartridge = cartridge::Cartridge::load(rom_data).unwrap();
    }
}
