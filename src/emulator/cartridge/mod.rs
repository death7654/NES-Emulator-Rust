#[derive(Clone, Copy)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
    SingleScreenLower,
    SingleScreenUpper,
}

pub trait Mapper {
    fn cpu_read(&self, address: u16) -> u8;
    fn cpu_write(&mut self, address: u16, data: u8);
    fn ppu_read(&self, address: u16) -> u8;
    fn ppu_write(&mut self, address: u16, data: u8);

    /// Mappers that control mirroring dynamically (MMC1, MMC3, etc.)
    /// override this. Mappers that don't (NROM) use the default, which
    /// signals "defer to the cartridge's header-derived mirroring."
    fn mirroring(&self) -> Option<Mirroring> {
        None
    }
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
            chr_is_ram,
        }
    }
}

impl Mapper for Mapper0 {
    fn cpu_read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0xFFFF => {
                let mut mapped_addr = (addr - 0x8000) as usize;
                mapped_addr %= self.prg_rom.len();
                if self.prg_banks == 1 {
                    mapped_addr %= 0x4000;
                }
                self.prg_rom[mapped_addr]
            }
            _ => 0,
        }
    }

    fn cpu_write(&mut self, _address: u16, _data: u8) {}

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

pub struct Mapper1 {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_ram: Vec<u8>,

    shift_register: u8,
    shift_count: u8,

    control_reg: u8,
    chr_bank_0: u8,
    chr_bank_1: u8,
    prg_bank: u8,

    chr_is_ram: bool,
}

impl Mapper1 {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, chr_is_ram: bool) -> Self {
        Self {
            prg_rom,
            chr_rom,
            // Allocate 8 KB (8192 bytes) of work/save RAM
            prg_ram: vec![0u8; 8192],

            // Shift register initialization
            shift_register: 0,
            shift_count: 0,

            // Control register defaults to 0x0C (PRG Mode 3: Fix last bank at 0xC000)
            control_reg: 0x0C,

            // Default bank selections
            chr_bank_0: 0,
            chr_bank_1: 0,
            prg_bank: 0,

            chr_is_ram,
        }
    }
}

impl Mapper for Mapper1 {
    fn cpu_read(&self, addr: u16) -> u8 {
        match addr {
            // Save/Work RAM (0x6000-0x7FFF)
            0x6000..=0x7FFF => self.prg_ram[(addr - 0x6000) as usize],

            // Cartridge PRG-ROM (0x8000-0xFFFF)
            0x8000..=0xFFFF => {
                let prg_mode = (self.control_reg >> 2) & 0x03;
                let last_bank = (self.prg_rom.len() / 0x4000) - 1;

                let bank = match prg_mode {
                    // Modes 0 & 1: switch 32 KB at a time, ignore bit 0 of prg_bank
                    0 | 1 => {
                        let base_bank = (self.prg_bank & 0x1E) as usize;
                        if addr < 0xC000 { base_bank } else { base_bank + 1 }
                    }
                    // Mode 2: fix bank 0 at 0x8000, switch 16 KB bank at 0xC000
                    2 => {
                        if addr < 0xC000 { 0 } else { (self.prg_bank & 0x0F) as usize }
                    }
                    // Mode 3: switch 16 KB bank at 0x8000, fix last bank at 0xC000
                    3 => {
                        if addr < 0xC000 { (self.prg_bank & 0x0F) as usize } else { last_bank }
                    }
                    _ => unreachable!(),
                };

                let offset = (addr & 0x3FFF) as usize;
                let mapped_addr = bank * 0x4000 + offset;
                self.prg_rom[mapped_addr % self.prg_rom.len()]
            }

            _ => 0,
        }
    }

    fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr {
            // Save/Work RAM (0x6000-0x7FFF)
            0x6000..=0x7FFF => {
                self.prg_ram[(addr - 0x6000) as usize] = data;
            }

            // MMC1 shift register (0x8000-0xFFFF)
            0x8000..=0xFFFF => {
                // Bit 7 set: reset shift register and force PRG mode to 3
                if (data & 0x80) != 0 {
                    self.shift_register = 0;
                    self.shift_count = 0;
                    self.control_reg |= 0x0C;
                } else {
                    // Shift LSB of data into position
                    self.shift_register |= (data & 0x01) << self.shift_count;
                    self.shift_count += 1;

                    // On the 5th write, copy the 5-bit payload to the target register
                    if self.shift_count == 5 {
                        let register_index = (addr >> 13) & 0x03;
                        let value = self.shift_register & 0x1F;

                        match register_index {
                            0 => self.control_reg = value,
                            1 => self.chr_bank_0 = value,
                            2 => self.chr_bank_1 = value,
                            3 => self.prg_bank = value,
                            _ => {}
                        }

                        self.shift_register = 0;
                        self.shift_count = 0;
                    }
                }
            }

            _ => {}
        }
    }

    fn ppu_read(&self, addr: u16) -> u8 {
        if addr >= 0x2000 || self.chr_rom.is_empty() {
            return 0;
        }

        let chr_mode = (self.control_reg >> 4) & 0x01;
        let bank = match chr_mode {
            // Mode 0: switch 8 KB at once (ignore bit 0 of chr_bank_0)
            0 => {
                let base_bank = (self.chr_bank_0 & 0x1E) as usize;
                if addr < 0x1000 { base_bank } else { base_bank + 1 }
            }
            // Mode 1: switch two independent 4 KB banks
            1 => {
                if addr < 0x1000 {
                    self.chr_bank_0 as usize
                } else {
                    self.chr_bank_1 as usize
                }
            }
            _ => unreachable!(),
        };

        let offset = (addr & 0x0FFF) as usize;
        let mapped_addr = bank * 0x1000 + offset;
        self.chr_rom[mapped_addr % self.chr_rom.len()]
    }

    fn ppu_write(&mut self, addr: u16, data: u8) {
        if addr >= 0x2000 || !self.chr_is_ram {
            return;
        }

        let chr_mode = (self.control_reg >> 4) & 0x01;
        let bank = match chr_mode {
            0 => {
                let base_bank = (self.chr_bank_0 & 0x1E) as usize;
                if addr < 0x1000 { base_bank } else { base_bank + 1 }
            }
            1 => {
                if addr < 0x1000 {
                    self.chr_bank_0 as usize
                } else {
                    self.chr_bank_1 as usize
                }
            }
            _ => unreachable!(),
        };

        let offset = (addr & 0x0FFF) as usize;
        let mapped_addr = bank * 0x1000 + offset;
        let len = self.chr_rom.len();
        if len > 0 {
            self.chr_rom[mapped_addr % len] = data;
        }
    }

    fn mirroring(&self) -> Option<Mirroring> {
        Some(match self.control_reg & 0x03 {
            0 => Mirroring::SingleScreenLower,
            1 => Mirroring::SingleScreenUpper,
            2 => Mirroring::Vertical,
            3 => Mirroring::Horizontal,
            _ => unreachable!(),
        })
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
        // Safety check for header
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

        // Advance through the file linearly
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

        // Slice CHR-ROM or set up CHR-RAM
        let (chr_rom, chr_is_ram) = if chr_banks == 0 {
            // 8KB of RAM allocated when no CHR banks are present in the file
            (vec![0u8; 8192], true)
        } else {
            (bytes[offset..offset + chr_size].to_vec(), false)
        };

        // Instantiate mapper
        let mapper: Box<dyn Mapper> = match mapper_id {
            0 => Box::new(Mapper0::new(prg_rom, chr_rom, chr_is_ram)),
            1 => Box::new(Mapper1::new(prg_rom, chr_rom, chr_is_ram)),
            _ => return Err(format!("Unsupported Mapper ID: {}", mapper_id)),
        };

        Ok(Self {
            mapper: Some(mapper),
            mirroring,
        })
    }

    pub fn map_nametable_address(&self, address: u16) -> usize {
        // Some mappers (MMC1, MMC3, etc.) control mirroring dynamically at
        // runtime and override whatever the iNES header originally said.
        // Fall back to the header-derived value if the mapper doesn't
        // implement dynamic mirroring.
        let mirroring = self
            .mapper
            .as_ref()
            .and_then(|m| m.mirroring())
            .unwrap_or(self.mirroring);

        let addr = (address - 0x2000) as usize;
        let table = addr / 0x400;
        let offset = addr % 0x400;

        let physical_table = match mirroring {
            Mirroring::Horizontal => {
                if table == 0 || table == 1 { 0 } else { 1 }
            }
            Mirroring::Vertical => {
                if table == 0 || table == 2 { 0 } else { 1 }
            }
            Mirroring::FourScreen => table % 2,
            Mirroring::SingleScreenLower => 0,
            Mirroring::SingleScreenUpper => 1,
        };

        physical_table * 0x400 + offset
    }
}