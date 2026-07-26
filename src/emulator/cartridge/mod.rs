#[derive(Clone, Copy)]
enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

pub struct Cartridge {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    chr_is_ram: bool,
    mirroring: Mirroring,
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            prg_rom: vec![0; 0x8000],
            chr_rom: vec![0; 0x8000],
            chr_is_ram: true,
            mirroring: Mirroring::Horizontal,
        }
    }

    pub fn from_ines(&mut self, data: &[u8]) {
        assert!(
            data.len() >= 16 && &data[0..4] == b"NES\x1A",
            "Not a valid iNES ROM file"
        );

        let prg_banks = data[4] as usize; // 16KB units
        let chr_banks = data[5] as usize; // 8KB units
        let flags6 = data[6];

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

        let prg_size = prg_banks * 16 * 1024;
        let prg_rom = data[offset..offset + prg_size].to_vec();
        offset += prg_size;

        let (chr_rom, chr_is_ram) = if chr_banks == 0 {
            // CHR banks == 0 means the cartridge uses CHR-RAM instead of
            // CHR-ROM; start it zeroed, writable via ppu_write.
            (vec![0u8; 8 * 1024], true)
        } else {
            let chr_size = chr_banks * 8 * 1024;
            (data[offset..offset + chr_size].to_vec(), false)
        };

        self.prg_rom = prg_rom;
        self.chr_rom = chr_rom;
        self.chr_is_ram = chr_is_ram;
        self.mirroring = mirroring;
    }

    pub fn ppu_read(&self, address: u16) -> u8 {
        let addr = address as usize % self.chr_rom.len();
        self.chr_rom[addr]
    }

    pub fn ppu_write(&mut self, address: u16, data: u8) {
        if self.chr_is_ram {
            let len = self.chr_rom.len();
            self.chr_rom[address as usize % len] = data;
        }
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
