#[derive(Clone, Copy)]
enum Mirroring {
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
                let mut mapped_addr = addr as usize;
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
        if &bytes[0..4] != b"NES\x1A" {
            return Err("Invalid iNES header".to_string());
        }

        let prg_size = bytes[4] as usize * 16384; // 16KB units
        let chr_size = bytes[5] as usize * 8192; // 8KB units

        let mapper_low = bytes[6] >> 4;
        let mapper_high = bytes[7] & 0xF0;
        let mapper_id = mapper_high | mapper_low;

        let prg_start = 16;
        let prg_end = prg_start + prg_size;
        let prg_rom = bytes[prg_start..prg_end].to_vec();

        let flags6 = bytes[6];
        let has_trainer = (flags6 & 0x04) != 0;

        let mirroring = if (flags6 & 0x08) != 0 {
            Mirroring::FourScreen
        } else if (flags6 & 0x01) != 0 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };

        let mut offset = 16;
        if has_trainer {
            offset += 512; // skip 512-byte trainer if present
        }

        let chr_banks = bytes[5] as usize;
        let (chr_rom, chr_is_ram) = if chr_banks == 0 {
            // CHR banks == 0 means the cartridge uses CHR-RAM instead of
            // CHR-ROM; start it zeroed, writable via ppu_write.
            (vec![0u8; 8 * 1024], true)
        } else {
            let chr_size = chr_banks * 8 * 1024;
            (bytes[offset..offset + chr_size].to_vec(), false)
        };

        // Factory selection based on Mapper ID
        let mapper: Box<dyn Mapper> = match mapper_id {
            0 => Box::new(Mapper0::new(prg_rom, chr_rom, chr_is_ram)),
            // 1 => Box::new(Mapper1::new(prg_rom, chr_rom)), // MMC1
            // 2 => Box::new(Mapper2::new(prg_rom, chr_rom)), // UxROM
            _ => return Err(format!("Unsupported Mapper ID: {}", mapper_id)),
        };

        Ok(Self {
            mapper: Some(mapper),
            mirroring: mirroring,
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
