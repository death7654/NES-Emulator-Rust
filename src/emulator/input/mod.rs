use sdl2::keyboard::{Keycode, Scancode::J};

pub struct Controller {
    a: bool,
    b: bool,
    select: bool,
    start: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,

    strobe: bool,
    shift: u8,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            a: false,
            b: false,
            select: false,
            start: false,
            up: false,
            down: false,
            left: false,
            right: false,
            strobe: false,
            shift: 0
        }
    }
    pub fn press(&mut self, key: Keycode) {
      println!("{key}");
        match key {
            // D-Pad (Supports both WASD and Arrow Keys)
            Keycode::W | Keycode::Up => self.up = true,
            Keycode::S | Keycode::Down => self.down = true,
            Keycode::A | Keycode::Left => self.left = true,
            Keycode::D | Keycode::Right => self.right = true,

            // Action Buttons (J/K or Z/X)
            Keycode::J | Keycode::Z => self.b = true,
            Keycode::K | Keycode::X => self.a = true,

            // Menu / System
            Keycode::RShift | Keycode::LShift | Keycode::Tab => self.select = true,
            Keycode::Return | Keycode::Space => self.start = true,

            _ => {}
        }
    }

    pub fn release(&mut self, key: Keycode) {
        match key {
            // D-Pad (Supports both WASD and Arrow Keys)
            Keycode::W | Keycode::Up => self.up = false,
            Keycode::S | Keycode::Down => self.down = false,
            Keycode::A | Keycode::Left => self.left = false,
            Keycode::D | Keycode::Right => self.right = false,

            // Action Buttons (J/K or Z/X)
            Keycode::J | Keycode::Z => self.b = false,
            Keycode::K | Keycode::X => self.a = false,

            // Menu / System
            Keycode::RShift | Keycode::LShift | Keycode::Tab => self.select = false,
            Keycode::Return | Keycode::Space => self.start = false,

            _ => {}
        }
    }

    fn snapshot(&self) -> u8 {
        let mut output: u8 = 0;

        if self.a {
            output |= 1 << 0;
        }
        if self.b {
            output |= 1 << 1;
        }
        if self.select {
            output |= 1 << 2;
        }
        if self.start {
            output |= 1 << 3;
        }
        if self.up {
            output |= 1 << 4;
        }
        if self.down {
            output |= 1 << 5;
        }
        if self.left {
            output |= 1 << 6;
        }
        if self.right {
            output |= 1 << 7;
        }
        return output;
    }

    pub fn write_strobe(&mut self, value: u8) {
        let strobe_bit = (value & 0x01) != 0;
        if strobe_bit {
            self.shift = self.snapshot();
        }
        self.strobe = strobe_bit;
    }


    pub fn get_input(&mut self) -> u8
    {
      if self.strobe
      {
        self.shift = self.snapshot();
        return self.shift & 0x01;
      }
      let bit = self.shift & 0x01;
      self.shift >>= 1;
      self.shift |= 0x80;
      bit
    }
}
