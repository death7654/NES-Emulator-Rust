use crate::emulator::cartridge::Cartridge;

pub const NES_SYSTEM_PALETTE: [u32; 64] = [
    0x808080, 0x003DA6, 0x0012B0, 0x440096, 0xA1005E, 0xC70028, 0xBA0600, 0x8C1700, 0x5C2F00,
    0x104500, 0x004A00, 0x00472E, 0x004166, 0x000000, 0x000000, 0x000000, 0xC5C5C5, 0x006DD7,
    0x2442FF, 0x7700E1, 0xD9009C, 0xFF014C, 0xF61E00, 0xC73E00, 0x9B6100, 0x4C7A00, 0x008400,
    0x008343, 0x007A8D, 0x000000, 0x000000, 0x000000, 0xFFFFFF, 0x64B0FF, 0x9290FF, 0xC676FF,
    0xF36AFF, 0xFE6EC9, 0xFF7A82, 0xFA9248, 0xCFAB00, 0x93C600, 0x51D300, 0x27D26E, 0x1ED2B2,
    0x4B4B4B, 0x000000, 0x000000, 0xFFFFFF, 0xC0E0FF, 0xD2D2FF, 0xE8C8FF, 0xFBC2FF, 0xFEC4EA,
    0xFECECA, 0xF5D0B2, 0xE4DCB2, 0xCCDDAE, 0xB5E1B2, 0xA1E2C3, 0x9CE2DF, 0xB9B9B9, 0x000000,
    0x000000,
];

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;
pub struct PPU {
    pub frame_buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    // ram
    pub vram: [u8; 2048],      // mapped as 0x2000 -> 0x2EFF
    pub palette_ram: [u8; 32], // mapped 0x3F00 -> 0x3F1F
    pub oam_ram: [u8; 256],
    vram_read_buffer: u8,
    ppu_bus_address: u16,

    // registers the cpu can access
    ppuctrl: u8,      // 0x2000, write only
    ppumask: u8,      // 0x2001, write only
    ppustatus: u8,    // 0x2002, Read Only
    oam_address: u16, // 0x2003 write only
    oam_data: u8,     // 0x2004, read write
    ppu_scroll: u8,   // 0x2005, write only
    ppu_address: u8,  // 0x2006 write only
    ppu_data: u8,     // 0x2007 read write

    oam_dma: u8,

    // internal registers
    v: u16,
    t: u16,
    x: u8,
    w: bool,

    ppu_dots: u16,  // 0 to 341
    scanlines: u16, // 0 to 261

    // temporary register
    latch_nametable_byte: u8,
    latch_attribute_byte: u8,
    latch_bg_low_byte: u8,
    latch_bg_high_byte: u8,

    bg_shift_pattern_low: u16,
    bg_shift_pattern_high: u16,
    bg_shift_attrib_low: u16,
    bg_shift_attrib_high: u16,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            frame_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            vram: [0; 2048],
            palette_ram: [0; 32],
            oam_ram: [0; 256],
            vram_read_buffer: 0,
            ppu_bus_address: 0,
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
            ppu_dots: 0,
            scanlines: 0,
            latch_nametable_byte: 0,
            latch_attribute_byte: 0,
            latch_bg_low_byte: 0,
            latch_bg_high_byte: 0,
            bg_shift_pattern_low: 0,
            bg_shift_pattern_high: 0,
            bg_shift_attrib_low: 0,
            bg_shift_attrib_high: 0,
        }
    }
    fn render_pixel(&mut self) {
        // Determine which bit in the high byte of the shift register we are reading
        let bit_mux: u16 = 15 - (self.x as u16);

        // Extract pattern bits (bitplanes 0 and 1)
        let p_bit0 = ((self.bg_shift_pattern_low >> bit_mux) & 0x01) as u8;
        let p_bit1 = ((self.bg_shift_pattern_high >> bit_mux) & 0x01) as u8;
        let pattern_color = (p_bit1 << 1) | p_bit0;

        // Extract palette ID bits
        let a_bit0 = ((self.bg_shift_attrib_low >> bit_mux) & 0x01) as u8;
        let a_bit1 = ((self.bg_shift_attrib_high >> bit_mux) & 0x01) as u8;
        let palette_id = (a_bit1 << 1) | a_bit0;

        // Resolve RAM palette index
        let palette_addr = if pattern_color == 0 {
            0
        } else {
            ((palette_id * 4) + pattern_color) as usize
        };

        let sys_palette_index = self.palette_ram[palette_addr] & 0x3F;
        let rgb_color = NES_SYSTEM_PALETTE[sys_palette_index as usize];

        // Plot to frame buffer
        let x = (self.ppu_dots - 1) as usize;
        let y = self.scanlines as usize;
        self.frame_buffer[y * SCREEN_WIDTH + x] = rgb_color;
    }

    fn shift_bg_registers(&mut self) {
        self.bg_shift_pattern_low <<= 1;
        self.bg_shift_pattern_high <<= 1;
        self.bg_shift_attrib_low <<= 1;
        self.bg_shift_attrib_high <<= 1;
    }

    pub fn step(&mut self, cartridge: &Cartridge) -> bool {
        let is_rendering_line = self.scanlines <= 239 || self.scanlines == 261;
        let is_visible_line = self.scanlines <= 239;

        if is_rendering_line {
            // Pixel Output & Shift Register Updates
            if is_visible_line && self.ppu_dots >= 1 && self.ppu_dots <= 256 {
                self.render_pixel();
                self.shift_bg_registers();
            }

            // Shifts also happen during tile pre-fetching at the end of the scanline!
            if (self.ppu_dots >= 321 && self.ppu_dots <= 336) {
                self.shift_bg_registers();
            }

            // Memory Fetches Loop
            if (self.ppu_dots >= 1 && self.ppu_dots <= 256)
                || (self.ppu_dots >= 321 && self.ppu_dots <= 336)
            {
                let mods = self.ppu_dots % 8;
                match mods {
                    1 => {
                        let address = 0x2000 | (self.v & 0x0FFF);
                        self.ppu_bus_address = address;
                    }
                    2 => {
                        self.latch_nametable_byte = self.ppu_read(cartridge, self.ppu_bus_address);
                    }
                    3 => {
                        let attribute_address = 0x23C0
                            | (self.v & 0x0C00)
                            | ((self.v >> 4) & 0x0038)
                            | ((self.v >> 2) & 0x0007);

                        self.ppu_bus_address = attribute_address;
                    }
                    4 => {
                        let attribute_byte = self.ppu_read(cartridge, self.ppu_bus_address);
                        let shift = ((self.v >> 4) & 0x04) | (self.v & 0x02);
                        self.latch_attribute_byte = (attribute_byte >> shift) & 0x03;
                    }
                    5 => {
                        let pattern_table_base = if (self.ppuctrl & 0x10) != 0 {
                            0x1000
                        } else {
                            0x0000
                        };
                        let fine_y = (self.v >> 12) & 0x07;
                        let pattern_addr =
                            pattern_table_base + ((self.latch_nametable_byte as u16) << 4) + fine_y;

                        self.ppu_bus_address = pattern_addr;
                    }
                    6 => {
                        self.latch_bg_low_byte = self.ppu_read(cartridge, self.ppu_bus_address);
                    }
                    7 => {
                        let pattern_table_base = if (self.ppuctrl & 0x10) != 0 {
                            0x1000
                        } else {
                            0x0000
                        };
                        let fine_y = (self.v >> 12) & 0x07;
                        let pattern_addr = pattern_table_base
                            + ((self.latch_nametable_byte as u16) << 4)
                            + fine_y
                            + 8;

                        self.ppu_bus_address = pattern_addr;
                    }
                    0 => {
                        self.latch_bg_high_byte = self.ppu_read(cartridge, self.ppu_bus_address);

                        // Load latched bytes into the bottom 8 bits of 16-bit registers
                        self.bg_shift_pattern_low =
                            (self.bg_shift_pattern_low & 0xFF00) | (self.latch_bg_low_byte as u16);
                        self.bg_shift_pattern_high = (self.bg_shift_pattern_high & 0xFF00)
                            | (self.latch_bg_high_byte as u16);

                        let attr_bit0 = if (self.latch_attribute_byte & 0x01) != 0 {
                            0xFF
                        } else {
                            0x00
                        };
                        let attr_bit1 = if (self.latch_attribute_byte & 0x02) != 0 {
                            0xFF
                        } else {
                            0x00
                        };

                        self.bg_shift_attrib_low = (self.bg_shift_attrib_low & 0xFF00) | attr_bit0;
                        self.bg_shift_attrib_high =
                            (self.bg_shift_attrib_high & 0xFF00) | attr_bit1;

                        self.increment_coarse_x();
                    }
                    _ => {}
                }
            }

            // Scroll Register Housekeeping
            if self.ppu_dots == 256 {
                self.increment_fine_y();
            }

            if self.ppu_dots == 257 {
                self.copy_x();
            }

            // Pre-render line (261) vertical scroll reset
            if self.scanlines == 261 && self.ppu_dots >= 280 && self.ppu_dots <= 304 {
                self.copy_y();
            }
        }

        // Dot and Scanline Advancement
        self.ppu_dots += 1;
        if self.ppu_dots >= 341 {
            self.ppu_dots = 0;
            self.scanlines += 1;

            if self.scanlines >= 262 {
                self.scanlines = 0;
                return true; // Frame complete
            }
        }

        false
    }
    pub fn increment_fine_y(&mut self) {
        if (self.v & 0x7000) != 0x7000 {
            self.v += 0x1000;
        } else {
            self.v &= !0x7000;

            let mut y = (self.v & 0x03E0) >> 5;

            if y == 29 {
                y = 0;
                self.v ^= 0x0800;
            } else if y == 31 {
                y = 0;
            } else {
                y += 1;
            }

            // Put coarse Y back into v
            self.v = (self.v & !0x03E0) | (y << 5);
        }
    }
    pub fn copy_y(&mut self) {
        let mask = 0x7BE0;
        self.v = (self.v & !mask) | (self.t & mask);
    }
    pub fn copy_x(&mut self) {
        let mask = 0x041F;
        self.v = (self.v & !mask) | (self.t & mask);
    }
    fn increment_coarse_x(&mut self) {
        if (self.v & 0x001F) == 31 {
            self.v &= !0x001F;
            self.v ^= 0x0400;
        } else {
            self.v += 1;
        }
    }

    pub fn ppu_read(&self, cartridge: &Cartridge, mut addr: u16) -> u8 {
        // PPU address bus is 14 bits wide
        addr &= 0x3FFF;

        match addr {
            // pattern tables
            0x0000..=0x1FFF => cartridge.ppu_read(addr),

            // Nametables
            0x2000..=0x3EFF => {
                let mirrored_addr = addr & 0x2FFF;

                let vram_index = cartridge.map_nametable_address(mirrored_addr);
                self.vram[vram_index]
            }

            // pallette ram
            0x3F00..=0x3FFF => {
                // Mask down to 32 bytes (0x3F00 - 0x3F1F)
                let mut palette_addr = (addr & 0x001F) as usize;
                // 0x3F10, 0x3F14, 0x3F18, 0x3F1C are mirrors of 0x3F00, 0x3F04, 0x3F08, 0x3F0C
                if palette_addr == 0x10
                    || palette_addr == 0x14
                    || palette_addr == 0x18
                    || palette_addr == 0x1C
                {
                    palette_addr -= 0x10;
                }

                self.palette_ram[palette_addr]
            }

            _ => 0,
        }
    }
    pub fn ppu_write(&mut self, cartridge: &mut Cartridge, mut addr: u16, data: u8) {
        addr &= 0x3FFF;

        match addr {
            // Pattern tables (handled by cartridge / mapper CHR RAM if present)
            0x0000..=0x1FFF => {
                cartridge.ppu_write(addr, data);
            }

            // Nametables
            0x2000..=0x3EFF => {
                let mirrored_addr = addr & 0x2FFF;
                let vram_index = cartridge.map_nametable_address(mirrored_addr);
                self.vram[vram_index] = data;
            }

            // Palette RAM
            0x3F00..=0x3FFF => {
                let mut palette_addr = (addr & 0x001F) as usize;
                if palette_addr == 0x10
                    || palette_addr == 0x14
                    || palette_addr == 0x18
                    || palette_addr == 0x1C
                {
                    palette_addr -= 0x10;
                }
                self.palette_ram[palette_addr] = data;
            }

            _ => {}
        }
    }

    pub fn cpu_read(&mut self, cartridge: &Cartridge, addr: u16) -> u8 {
        // Registers 0x2000-0x2007 mirror every 8 bytes up to 0x3FFF
        let reg = addr % 8;

        match reg {
            2 => {
                let status = self.ppustatus;

                // Reading clears bit 7 (VBlank flag)
                self.ppustatus &= !0x80;

                // Reading resets the double-write latch
                self.w = false;

                status
            }
            4 => self.oam_ram[self.oam_address as usize],
            7 => {
                let current_vram_addr = self.v & 0x3FFF;
                let data = self.ppu_read(cartridge, current_vram_addr);

                // Increment VRAM address depending on bit 2 of PPUCTRL
                let increment = if (self.ppuctrl & 0x04) != 0 { 32 } else { 1 };
                self.v = self.v.wrapping_add(increment);

                // Palette reads are unbuffered, standard reads use the latch buffer
                if current_vram_addr >= 0x3F00 {
                    self.vram_read_buffer = self.ppu_read(cartridge, current_vram_addr - 0x1000);
                    data
                } else {
                    let buffered = self.vram_read_buffer;
                    self.vram_read_buffer = data;
                    buffered
                }
            }

            // 0x2000, 0x2001, 0x2003, 0x2005, 0x2006 are write-only
            _ => 0,
        }
    }
    pub fn cpu_write(&mut self, cartridge: &mut Cartridge, addr: u16, data: u8) {
        let reg = addr % 8;

        match reg {
            // 0x2000, set ppu controls
            0 => {
                self.ppuctrl = data;
                // Update base nametable selection bits in temporary address 't'
                self.t = (self.t & !0x0C00) | (((data as u16) & 0x03) << 10);
            }

            // 0x2001, set ppu masks
            1 => {
                self.ppumask = data;
            }

            // 0x2003, set oam address
            3 => {
                self.oam_address = data as u16;
            }

            // 0x2004, set oam data
            4 => {
                self.oam_ram[self.oam_address as usize] = data;
                self.oam_address = (self.oam_address + 1) & 0xFF; // Auto-increment OAM address
            }

            // 0x2005, update ppuscroll writes X scroll then Y scroll into t
            5 => {
                if !self.w {
                    // first write, Fine X and Coarse X
                    self.x = data & 0x07;
                    self.t = (self.t & !0x001F) | ((data as u16) >> 3);
                    self.w = true;
                } else {
                    // second write, Fine Y and Coarse Y
                    self.t = (self.t & !0x73E0)
                        | (((data as u16) & 0x07) << 12)
                        | (((data as u16) & 0xF8) << 2);
                    self.w = false;
                }
            }

            // 0x2006, ppu address, writes high byte then low byte into t, then updates v
            6 => {
                if !self.w {
                    // First write, writes high byte of VRAM address
                    self.t = (self.t & !0x7F00) | (((data as u16) & 0x3F) << 8);
                    self.w = true;
                } else {
                    // Second write, writes the low byte of VRAM address
                    self.t = (self.t & !0x00FF) | (data as u16);
                    self.v = self.t; // Copy 't' to active VRAM address 'v'
                    self.w = false;
                }
            }

            // 0x2007 update ppu data
            7 => {
                let current_vram_addr = self.v & 0x3FFF;
                self.ppu_write(cartridge, current_vram_addr, data);

                // Increment VRAM address depending on bit 2 of PPUCTRL
                let increment = if (self.ppuctrl & 0x04) != 0 { 32 } else { 1 };
                self.v = self.v.wrapping_add(increment);
            }

            _ => {
                // 0x2002 is read only
            }
        }
    }
}
