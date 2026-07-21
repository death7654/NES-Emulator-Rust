pub struct PPU {
    // registers the cpu can access
    ppuctrl: u8,     // 0x2000, write only
    ppumask: u8,     // 0x2001, write only
    ppustatus: u8,   // 0x2002, Read Only
    oam_address: u8, // 0x2003 write only
    oam_data: u8,    // 0x2004, read write
    ppu_scroll: u8,  // 0x2005, write only
    ppu_address: u8, // 0x2006 write only
    ppu_data: u8,    // 0x2007 read write

    oam_dma: u8,

    // internal registers
    v: u8,
    t: u8,
    x: u8,
    w: bool,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oam_address: 0,
            oam_data: 0,
            ppu_scroll: 0,
            ppu_address: 0,
            ppu_data: 0,
            oam_dma: 0,
            v: 0,
            t: 0,
            x: 0,
            w: false,
        }
    }

    pub fn step(&mut self, chr_rom: &[u8]) -> bool {
        return false;
    }
}
