pub struct Cartridge {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            prg_rom: vec![0; 0x8000],
            chr_rom: vec![0; 0x8000],
        }
    }

    pub fn ppu_read(&self, address: u16) -> u8 {
        return 0;
    }

    pub fn ppu_write(&mut self, address: u16, data: u8) {}

    pub fn map_nametable_address(&self, address: u16) -> usize {
        return 0;
    }
}
