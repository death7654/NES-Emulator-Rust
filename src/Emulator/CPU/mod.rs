use std::ptr::read;

use crate::Emulator::{CPU::AddressingModes::ZeroPage, bus::Bus};

mod registers;
pub struct CPU {
    registers: registers::Registers,
    cycles: u32,
}

struct AddrResult {
    data: u8,
    page_crossed: bool,
}

pub enum AddressingModes {
    Immediate,
    Absolute,
    XIndexedAbsolute,
    YIndexedAbsolute,
    ZeroPage,
    XIndexedZeroPage,
    YIndexedZeroPage,
    XIndexedZeroPageIndirect,
    ZeroPageIndirectYPaged,
}

pub enum LoadRegisters {
    A,
    X,
    y,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: registers::Registers::new(),
            cycles: 0,
        }
    }

    pub fn fetch(&mut self, bus: &mut Bus) -> u8 {
        let data = bus.read(self.registers.get_pc());

        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.tick();
        return data;
    }

    pub fn tick(&mut self) {
        self.cycles += 1;
    }

    pub fn execute(&mut self, opcode: u8, bus: &mut Bus) {
        let cc = opcode & 0x03; // Lower 2 bits  (00000011)
        let bbb = (opcode >> 2) & 0x07; // Middle 3 bits (00011100)
        let aaa = (opcode >> 5) & 0x07; // Upper 3 bits  (11100000)

        match cc {
            0b00 => {
                match bbb {
                    0b000 => match aaa {
                        0b101 => {
                            self.ld(AddressingModes::Immediate, LoadRegisters::y, bus);
                        }
                        _ => {
                            println!("{aaa}{bbb}{cc} not implemented (aaa)");
                        }
                    },
                    0b001 => match aaa {
                        0b101 => {
                            self.ld(AddressingModes::ZeroPage, LoadRegisters::y, bus);
                        }
                        _ => {
                            println!("{aaa}{bbb}{cc} not implemented (aaa)");
                        }
                    },
                    0b010 => {}
                    0b011 => match aaa {
                        0b101 => {
                            self.ld(AddressingModes::Absolute, LoadRegisters::y, bus);
                        }
                        _ => {
                            println!("{aaa}{bbb}{cc} not implemented (aaa)");
                        }
                    },
                    0b100 => match aaa {
                        0b000 => {
                            self.bpl(bus);
                        }
                        0b001 => {
                            self.bmi(bus);
                        }
                        0b010 => {
                            self.bvc(bus);
                        }
                        0b011 => {
                            self.bvs(bus);
                        }
                        0b100 => {
                            self.bcc(bus);
                        }
                        0b101 => {
                            self.bcs(bus);
                        }
                        0b110 => {
                            self.bne(bus);
                        }
                        0b111 => {
                            self.beq(bus);
                        }
                        _ => {
                            println!("{aaa}{bbb}{cc} not implemented (aaa)");
                        }
                    },
                    0b101 => match aaa {
                        0b101 => {
                            self.ld(AddressingModes::XIndexedZeroPage, LoadRegisters::y, bus);
                        }
                        _ => {
                            println!("{aaa}{bbb}{cc} not implemented (aaa)");
                        }
                    },
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
                    0b111 => match aaa {
                        0b101 => {
                            self.ld(AddressingModes::XIndexedAbsolute, LoadRegisters::y, bus);
                        }
                        _ => {
                            println!("{aaa}{bbb}{cc} not implemented (aaa)");
                        }
                    },
                    _ => {
                        println!("{bbb}{cc} not implemented (bbb)");
                    }
                }
            }
            0b01 => match bbb {
                0b000 => match aaa {
                    0b101 => {
                        self.ld(
                            AddressingModes::XIndexedZeroPageIndirect,
                            LoadRegisters::A,
                            bus,
                        );
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                0b001 => match aaa {
                    0b101 => {
                        self.ld(AddressingModes::ZeroPage, LoadRegisters::A, bus);
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                0b010 => match aaa {
                    0b101 => {
                        self.ld(AddressingModes::Immediate, LoadRegisters::A, bus);
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                0b011 => match aaa {
                    0b101 => {
                        self.ld(AddressingModes::Absolute, LoadRegisters::A, bus);
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                0b100 => match aaa {
                    0b101 => {
                        self.ld(
                            AddressingModes::ZeroPageIndirectYPaged,
                            LoadRegisters::A,
                            bus,
                        );
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                0b101 => match aaa {
                    0b101 => {
                        self.ld(AddressingModes::XIndexedZeroPage, LoadRegisters::A, bus);
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                0b110 => match aaa {
                    0b101 => {
                        self.ld(AddressingModes::YIndexedAbsolute, LoadRegisters::A, bus);
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                0b111 => match aaa {
                    0b101 => {
                        self.ld(AddressingModes::XIndexedAbsolute, LoadRegisters::A, bus);
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                _ => {
                    println!("{bbb}{cc} not implemented (bbb)");
                }
            },
            0b10 => match bbb {
                0b000 => match aaa {
                    0b101 => {
                        self.ld(AddressingModes::Immediate, LoadRegisters::X, bus);
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                0b001 => match aaa {
                    0b101 => {
                        self.ld(AddressingModes::ZeroPage, LoadRegisters::X, bus);
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                0b010 => {}
                0b011 => match aaa {
                    0b101 => {
                        self.ld(AddressingModes::Absolute, LoadRegisters::X, bus);
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                0b100 => {}
                0b101 => match aaa {
                    0b101 => {
                        self.ld(AddressingModes::YIndexedZeroPage, LoadRegisters::X, bus);
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
                0b110 => {}
                0b111 => match aaa {
                    0b101 => {
                        self.ld(AddressingModes::YIndexedAbsolute, LoadRegisters::X, bus);
                    }
                    _ => {
                        println!("{aaa}{bbb}{cc} not implemented (aaa)");
                    }
                },
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
    fn bcc(&mut self, bus: &mut Bus) {
        self.execute_branch(!self.registers.carry, bus);
    }

    // branch when there is a carry
    fn bcs(&mut self, bus: &mut Bus) {
        self.execute_branch(self.registers.carry, bus);
    }

    // branch when both operands are equal
    fn beq(&mut self, bus: &mut Bus) {
        self.execute_branch(self.registers.zero, bus);
    }

    // branch when the result is negative
    fn bmi(&mut self, bus: &mut Bus) {
        self.execute_branch(self.registers.negative, bus);
    }

    // branch when both operands are not equal
    fn bne(&mut self, bus: &mut Bus) {
        self.execute_branch(!self.registers.zero, bus);
    }

    // branch when the result is not negative
    fn bpl(&mut self, bus: &mut Bus) {
        self.execute_branch(!self.registers.negative, bus);
    }

    // branch when thee is no overflow
    fn bvc(&mut self, bus: &mut Bus) {
        self.execute_branch(!self.registers.overflow, bus);
    }

    // branch when there is an overflow
    fn bvs(&mut self, bus: &mut Bus) {
        self.execute_branch(self.registers.overflow, bus);
    }

    // execute branches
    fn execute_branch(&mut self, should_branch: bool, bus: &mut Bus) {
        let offset: i8 = self.fetch(bus) as i8;

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

    // load implementation

    fn get_addressing_mode(
        &mut self,
        addressing_mode: AddressingModes,
        bus: &mut Bus,
    ) -> AddrResult {
        let output: u8;
        let mut page_crossed = false;
        match addressing_mode {
            AddressingModes::Immediate => {
                output = self.fetch(bus);
            }
            AddressingModes::Absolute => {
                let lower = self.fetch(bus);
                self.tick();
                let upper = self.fetch(bus);
                self.tick();

                let address = ((upper as u16) << 8) | (lower as u16);
                output = bus.read(address);
            }
            AddressingModes::XIndexedAbsolute => {
                let lower = self.fetch(bus);
                self.tick();
                let upper = self.fetch(bus);
                self.tick();

                let x = self.registers.get_x();
                let base_address = ((upper as u16) << 8) | (lower as u16);
                let address = base_address.wrapping_add(x as u16);

                page_crossed = (base_address & 0xFF00) != (address & 0xFF00);

                output = bus.read(address);
                self.tick();
            }
            AddressingModes::YIndexedAbsolute => {
                let lower = self.fetch(bus);
                self.tick();
                let upper = self.fetch(bus);
                self.tick();

                let y = self.registers.get_y();
                let base_address = (upper as u16) << 8 | (lower as u16);
                let address = base_address.wrapping_add(y as u16);

                page_crossed = (base_address & 0xFF00) != (address & 0xFF00);

                output = bus.read(address);
                self.tick();
            }
            AddressingModes::ZeroPage => {
                let address = self.fetch(bus) as u16;

                output = bus.read(address);
                self.tick();
            }
            AddressingModes::XIndexedZeroPage => {
                let base_address = self.fetch(bus);
                self.tick();
                let x = self.registers.get_x();
                let address = base_address.wrapping_add(x) as u16;

                output = bus.read(address);
                self.tick();
            }
            AddressingModes::YIndexedZeroPage => {
                let base_address = self.fetch(bus);
                self.tick();
                let y = self.registers.get_x();
                let address = base_address.wrapping_add(y) as u16;

                output = bus.read(address);
                self.tick();
            }
            AddressingModes::XIndexedZeroPageIndirect => {
                let base_zp = self.fetch(bus);
                self.tick();

                let x = self.registers.get_x();
                let ptr = base_zp.wrapping_add(x);

                let lower = bus.read(ptr as u16);
                self.tick();
                let upper = bus.read(ptr.wrapping_add(1) as u16);
                self.tick();

                let address = ((upper as u16) << 8) | (lower as u16);
                output = bus.read(address);
                self.tick();
            }
            AddressingModes::ZeroPageIndirectYPaged => {
                let ptr = self.fetch(bus) as u16;

                let lower = bus.read(ptr);
                self.tick();
                let upper = bus.read((ptr & 0x00FF) | ((ptr + 1) & 0x00FF));
                self.tick();

                let base_address = ((upper as u16) << 8) | (lower as u16);
                let y = self.registers.get_y();
                let address = base_address.wrapping_add(y as u16);

                // Set flag if page crossed, but let the caller handle the self.tick() penalty
                page_crossed = (base_address & 0xFF00) != (address & 0xFF00);

                output = bus.read(address);
                self.tick();
            }
        }

        AddrResult {
            data: output,
            page_crossed,
        }
    }

    fn load_into_register(&mut self, register: LoadRegisters, data: u8) {
        match register {
            LoadRegisters::A => self.registers.set_a(data),
            LoadRegisters::X => self.registers.set_x(data),
            LoadRegisters::y => self.registers.set_y(data),
        }
    }

    fn ld(&mut self, addressing_mode: AddressingModes, register: LoadRegisters, bus: &mut Bus) {
        let result = self.get_addressing_mode(addressing_mode, bus);

        if result.page_crossed {
            self.tick();
        }

        self.load_into_register(register, result.data);
        self.set_zero(result.data);
        self.set_negative(result.data);
    }

    // easy flag sets
    fn set_zero(&mut self, data: u8) {
        if data == 0 {
            self.registers.zero = true;
        } else {
            self.registers.zero = false;
        }
    }

    fn set_negative(&mut self, data: u8) {
        if (data & 0x80) == 0x80 {
            self.registers.negative = true;
        } else {
            self.registers.negative = false;
        }
    }
}
