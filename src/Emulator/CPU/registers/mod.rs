pub struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pub pc: u16,
    pub sp: u16,

    // statuses
    pub negative: bool,
    pub overflow: bool,

    pub breakins: bool,
    pub decimal: bool,
    pub interrupt_disable: bool,
    pub zero: bool,
    pub carry: bool,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            negative: false,
            overflow: false,
            breakins: false,
            decimal: false,
            interrupt_disable: false,
            zero: false,
            carry: false,
        }
    }
    pub fn get_a(&self) -> u8 {
        return self.a;
    }
    pub fn set_a(&mut self, val: u8) {
        self.a = val;
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn set_x(&mut self, val: u8) {
        self.x = val;
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn set_y(&mut self, val: u8) {
        self.y = val;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, val: u16) {
        self.sp = val;
    }

    pub fn get_status(&self) -> u8 {
        let mut ret: u8 = 0;

        if self.negative {
            ret = ret | (1 << 7);
        }

        if self.overflow {
            ret = ret | (1 << 6);
        }

        ret = ret | (1 << 5);

        if self.breakins {
            ret = ret | (1 << 4);
        }

        if self.decimal {
            ret = ret | (1 << 3);
        }

        if self.interrupt_disable {
            ret = ret | (1 << 2);
        }

        if self.zero {
            ret = ret | (1 << 1);
        }

        if self.carry {
            ret = ret | (1 << 0);
        }

        return ret;
    }
}
