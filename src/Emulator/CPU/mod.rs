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

    pub fn fetch(&mut self) -> u8 {
        self.tick();
        return 0;
    }

    pub fn tick(&mut self) {
        self.registers.pc += 1
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
                    0b100 => match aaa {
                        0b000 => {
                            self.bpl();
                        }
                        0b001 => {
                            self.bmi();
                        }
                        0b010 => {
                            self.bvc();
                        }
                        0b011 => {
                            self.bvs();
                        }
                        0b100 => {
                            self.bcc();
                        }
                        0b101 => {
                            self.bcs();
                        }
                        0b110 => {
                            self.bne();
                        }
                        0b111 => {
                            self.beq();
                        }
                        _ => {
                            println!("{aaa}{bbb}{cc} not implemented (aaa)");
                        }
                    },
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
    // set decimal flag to true
    fn sed(&mut self) {
        self.registers.decimal = true;
        self.tick();
    }
    // set i flag to true
    fn sei(&mut self) {
        self.registers.interrupt_disable = true;
        self.tick();
    }

    // do nothing
    fn nop(&mut self) {
        self.tick();
    }

    // branch on when there is no carry
    fn bcc(&mut self) {
        self.execute_branch(!self.registers.carry);
    }

    // branch when there is a carry
    fn bcs(&mut self) {
        self.execute_branch(self.registers.carry);
    }

    // branch when both operands are equal
    fn beq(&mut self) {
        self.execute_branch(self.registers.zero);
    }

    // branch when the result is negative
    fn bmi(&mut self) {
        self.execute_branch(self.registers.negative);
    }

    // branch when both operands are not equal
    fn bne(&mut self) {
        self.execute_branch(!self.registers.zero);
    }

    // branch when the result is not negative
    fn bpl(&mut self) {
        self.execute_branch(!self.registers.negative);
    }

    // branch when thee is no overflow
    fn bvc(&mut self) {
        self.execute_branch(!self.registers.overflow);
    }

    // branch when there is an overflow
    fn bvs(&mut self) {
        self.execute_branch(self.registers.overflow);
    }

    // execute branches
    fn execute_branch(&mut self, should_branch: bool) {
        let offset: i8 = self.fetch() as i8;

        if should_branch {
            self.tick();

            let old_pc = self.registers.pc;
            let new_pc = (old_pc as i32).wrapping_add(offset as i32) as u16;

            self.registers.pc = new_pc;

            if (old_pc & 0xFF00) != (new_pc & 0xFF00) {
                self.tick();
            }
        }
    }
}
