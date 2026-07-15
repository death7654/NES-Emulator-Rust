use std::default;

mod registers;

pub struct CPU {
    registers: registers::Registers,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: registers::Registers::new(),
        }
    }

    pub fn fetch() -> u8 {
        return 0;
    }

    pub fn tick(&mut self) {
        self.registers.pc += 4
    }

    pub fn execute(&mut self, opcode: u8) {
        let cc = opcode & 0x03; // Lower 2 bits  (00000011)
        let bbb = (opcode >> 2) & 0x07; // Middle 3 bits (00011100)
        let aaa = (opcode >> 5) & 0x07; // Upper 3 bits  (11100000)

        match cc {
            0b00 => {
                match bbb {
                    0b000 => {}
                    0b001 => {}
                    0b010 => {}
                    0b011 => {}
                    0b100 => {}
                    0b101 => {}
                    0b110 => {
                        // set and clear flags group
                        match aaa {
                            0b000 => {
                                self.clc();
                            }
                            0b001 => {
                                self.sec();
                            }
                            0b010 => {
                                self.cli();
                            }
                            0b011 => {
                                self.sei();
                            }
                            0b100 => {
                                self.nop();
                            }
                            0b101 => {
                                self.clv();
                            }
                            0b110 => {
                                self.cld();
                            }
                            0b111 => {
                                self.sed();
                            }
                            _ => {
                                println!("{aaa}{bbb}{cc} not implemented (aaa)");
                            }
                        }
                    }
                    0b111 => {}
                    _ => {
                        println!("{bbb}{cc} not implemented (bbb)");
                    }
                }
            }
            0b01 => match bbb {
                0b000 => {}
                0b001 => {}
                0b010 => {}
                0b011 => {}
                0b100 => {}
                0b101 => {}
                0b110 => {}
                0b111 => {}
                _ => {
                    println!("{bbb}{cc} not implemented (bbb)");
                }
            },
            0b10 => match bbb {
                0b000 => {}
                0b001 => {}
                0b010 => {}
                0b011 => {}
                0b100 => {}
                0b101 => {}
                0b110 => {}
                0b111 => {}
                _ => {
                    println!("{bbb}{cc} not implemented (bbb)");
                }
            },
            0b11 => match bbb {
                0b000 => {}
                0b001 => {}
                0b010 => {}
                0b011 => {}
                0b100 => {}
                0b101 => {}
                0b110 => {}
                0b111 => {}
                _ => {
                    println!("{bbb}{cc} not implemented (bbb)");
                }
            },
            _ => {
                println!("{cc} not implemented (cc)");
            }
        }
    }

    // clear carry flag
    fn clc(&mut self) {
        self.registers.carry = false;
        self.tick();
    }

    // clear decimal flag
    fn cld(&mut self) {
        self.registers.decimal = false;
        self.tick();
    }

    // clear interrupt disable flag
    fn cli(&mut self) {
        self.registers.interrupt_disable = false;
        self.tick();
    }

    // clear zero flag
    fn clv(&mut self) {
        self.registers.zero = false;
        self.tick();
    }

    // set clear flag
    fn sec(&mut self) {
        self.registers.carry = true;
        self.tick();
    }

    fn sed(&mut self) {
        self.registers.decimal = true;
        self.tick();
    }

    fn sei(&mut self) {
        self.registers.interrupt_disable = true;
        self.tick();
    }

    fn nop(&mut self) {
        self.tick();
    }
}
