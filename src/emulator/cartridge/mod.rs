#[derive(Clone, Copy)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

pub trait Mapper {
    fn cpu_read(&self, address: u16) -> u8;
    fn cpu_write(&mut self, address: u16, data: u8);
    fn ppu_read(&self, address: u16) -> u8;
    fn ppu_write(&mut self, address: u16, data: u8);
}

pub struct Mapper0 {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_banks: u8, // 1 for 16KB, 2 for 32KB
    chr_is_ram: bool,
}

impl Mapper0 {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, chr_is_ram: bool) -> Self {
        let prg_banks = (prg_rom.len() / 0x4000) as u8;
        Self {
            prg_rom,
            chr_rom,
            prg_banks,
            chr_is_ram: chr_is_ram,
        }
    }
}

impl Mapper for Mapper0 {
    fn cpu_read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0xFFFF => {
                let mut mapped_addr = (addr - 0x8000) as usize;
                mapped_addr = mapped_addr as usize % self.prg_rom.len();
                if self.prg_banks == 1 {
                    mapped_addr %= 0x4000;
                }
                self.prg_rom[mapped_addr]
            }
            _ => 0,
        }
    }

    fn cpu_write(&mut self, address: u16, data: u8) {}

    fn ppu_read(&self, addr: u16) -> u8 {
        let addr = addr as usize % self.chr_rom.len();
        self.chr_rom[addr]
    }

    fn ppu_write(&mut self, address: u16, data: u8) {
        if self.chr_is_ram {
            let len = self.chr_rom.len();
            self.chr_rom[address as usize % len] = data;
        }
    }
}

pub struct Cartridge {
    pub mapper: Option<Box<dyn Mapper>>,
    pub mirroring: Mirroring,
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            mapper: None,
            mirroring: Mirroring::Horizontal,
        }
    }
    pub fn load(bytes: &[u8]) -> Result<Self, String> {
        // 1. Safety check for header
        if bytes.len() < 16 || &bytes[0..4] != b"NES\x1A" {
            return Err("Invalid iNES header".to_string());
        }

        let prg_size = bytes[4] as usize * 16384; // 16KB units
        let chr_banks = bytes[5] as usize;
        let chr_size = chr_banks * 8192; // 8KB units

        let flags6 = bytes[6];
        let flags7 = bytes[7];

        let mapper_low = flags6 >> 4;
        let mapper_high = flags7 & 0xF0;
        let mapper_id = mapper_high | mapper_low;

        let has_trainer = (flags6 & 0x04) != 0;

        let mirroring = if (flags6 & 0x08) != 0 {
            Mirroring::FourScreen
        } else if (flags6 & 0x01) != 0 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };

        // dvance through the file linearly
        let mut offset = 16; // Skip header

        if has_trainer {
            offset += 512; // Skip 512-byte trainer if present
        }

        let expected_chr_size = if chr_banks > 0 { chr_size } else { 0 };
        if bytes.len() < offset + prg_size + expected_chr_size {
            return Err(
                "File truncated: actual length is smaller than header specification".to_string(),
            );
        }

        // Slice PRG-ROM and advance offset past it
        let prg_rom = bytes[offset..offset + prg_size].to_vec();
        offset += prg_size;

        // Slice CHR-ROM  or set up CHR-RAM
        let (chr_rom, chr_is_ram) = if chr_banks == 0 {
            // 8KB of RAM allocated when no CHR banks are present in the file
            (vec![0u8; 8192], true)
        } else {
            (bytes[offset..offset + chr_size].to_vec(), false)
        };

        // Instantiate mapper
        let mapper: Box<dyn Mapper> = match mapper_id {
            0 => Box::new(Mapper0::new(prg_rom, chr_rom, chr_is_ram)),
            _ => return Err(format!("Unsupported Mapper ID: {}", mapper_id)),
        };

        Ok(Self {
            mapper: Some(mapper),
            mirroring,
        })
    }
    pub fn map_nametable_address(&self, address: u16) -> usize {
        let addr = (address - 0x2000) as usize;
        let table = addr / 0x400;
        let offset = addr % 0x400;

        let physical_table = match self.mirroring {
            Mirroring::Horizontal => {
                if table == 0 || table == 1 {
                    0
                } else {
                    1
                }
            }
            Mirroring::Vertical => {
                if table == 0 || table == 2 {
                    0
                } else {
                    1
                }
            }
            Mirroring::FourScreen => table % 2,
        };

        physical_table * 0x400 + offset
    }
}
