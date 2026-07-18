use std::ptr::read;

use crate::Emulator::{CPU::AddressingModes::ZeroPage, bus::Bus};

mod registers;
pub struct CPU {
    registers: registers::Registers,
    cycles: u32,
}

pub struct AddrResult {
    pub address: u16,
    pub page_crossed: bool,
}

pub enum AddressingModes {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    XIndexedZeroPageIndirect, // Also known as (Indirect, X)
    ZeroPageIndirectYPaged,   // Also known as (Indirect), Y
    Indirect,                 // Used exclusively by JMP
    NoneAddressing,
}

pub enum LoadRegisters {
    A,
    X,
    Y,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: registers::Registers::new(),
            cycles: 0,
        }
    }

    pub fn fetch(&mut self, bus: &mut Bus) -> u8 {
        let data = self.read_bus(bus, self.registers.get_pc());
        self.registers.pc = self.registers.pc.wrapping_add(1);

        return data;
    }

    fn read_bus(&mut self, bus: &mut Bus, address: u16) -> u8 {
        self.cycles += 1;
        bus.read(address)
    }

    fn write_bus(&mut self, bus: &mut Bus, address: u16, data: u8) {
        self.cycles += 1;
        bus.write(address, data);
    }

    pub fn tick(&mut self) {
        self.cycles += 1;
    }

    pub fn execute(&mut self, opcode: u8, bus: &mut Bus) {
        let cc = opcode & 0x03; // Lower 2 bits
        let bbb = (opcode >> 2) & 0x07; // Middle 3 bits
        let aaa = (opcode >> 5) & 0x07; // Upper 3 bits

        match (cc, aaa, bbb) {
            (0b00, 0b101, 0b000) => self.ld(AddressingModes::Immediate, LoadRegisters::Y, bus),
            (0b00, 0b101, 0b001) => self.ld(AddressingModes::ZeroPage, LoadRegisters::Y, bus),
            (0b00, 0b101, 0b011) => self.ld(AddressingModes::Absolute, LoadRegisters::Y, bus),
            (0b00, 0b101, 0b101) => self.ld(AddressingModes::ZeroPageX, LoadRegisters::Y, bus),
            (0b00, 0b101, 0b111) => self.ld(AddressingModes::AbsoluteX, LoadRegisters::Y, bus),

            (0b00, 0b000, 0b100) => self.bpl(bus),
            (0b00, 0b001, 0b100) => self.bmi(bus),
            (0b00, 0b010, 0b100) => self.bvc(bus),
            (0b00, 0b011, 0b100) => self.bvs(bus),
            (0b00, 0b100, 0b100) => self.bcc(bus),
            (0b00, 0b101, 0b100) => self.bcs(bus),
            (0b00, 0b110, 0b100) => self.bne(bus),
            (0b00, 0b111, 0b100) => self.beq(bus),

            (0b00, 0b000, 0b010) => self.php(bus),
            (0b00, 0b001, 0b010) => self.plp(bus),
            (0b00, 0b010, 0b010) => self.pha(bus),
            (0b00, 0b011, 0b010) => self.pla(bus),
            (0b00, 0b100, 0b010) => self.dey(),
            (0b00, 0b101, 0b010) => self.tay(),
            (0b00, 0b110, 0b010) => self.iny(),
            (0b00, 0b111, 0b010) => self.inx(),

            (0b00, 0b000, 0b110) => self.clc(),
            (0b00, 0b001, 0b110) => self.sec(),
            (0b00, 0b010, 0b110) => self.cli(),
            (0b00, 0b011, 0b110) => self.sei(),
            (0b00, 0b100, 0b110) => self.tya(),
            (0b00, 0b101, 0b110) => self.clv(),
            (0b00, 0b110, 0b110) => self.cld(),
            (0b00, 0b111, 0b110) => self.sed(),

            (0b01, 0b101, 0b000) => self.ld(
                AddressingModes::XIndexedZeroPageIndirect,
                LoadRegisters::A,
                bus,
            ),
            (0b01, 0b101, 0b001) => self.ld(AddressingModes::ZeroPage, LoadRegisters::A, bus),
            (0b01, 0b101, 0b010) => self.ld(AddressingModes::Immediate, LoadRegisters::A, bus),
            (0b01, 0b101, 0b011) => self.ld(AddressingModes::Absolute, LoadRegisters::A, bus),
            (0b01, 0b101, 0b100) => self.ld(
                AddressingModes::ZeroPageIndirectYPaged,
                LoadRegisters::A,
                bus,
            ),
            (0b01, 0b101, 0b101) => self.ld(AddressingModes::ZeroPageX, LoadRegisters::A, bus),
            (0b01, 0b101, 0b110) => self.ld(AddressingModes::AbsoluteY, LoadRegisters::A, bus),
            (0b01, 0b101, 0b111) => self.ld(AddressingModes::AbsoluteX, LoadRegisters::A, bus),

            (0b10, 0b101, 0b000) => self.ld(AddressingModes::Immediate, LoadRegisters::X, bus),
            (0b10, 0b101, 0b001) => self.ld(AddressingModes::ZeroPage, LoadRegisters::X, bus),
            (0b10, 0b101, 0b011) => self.ld(AddressingModes::Absolute, LoadRegisters::X, bus),
            (0b10, 0b101, 0b101) => self.ld(AddressingModes::ZeroPageY, LoadRegisters::X, bus),
            (0b10, 0b101, 0b111) => self.ld(AddressingModes::AbsoluteY, LoadRegisters::X, bus),

            (0b10, 0b111, 0b010) => self.nop(),

            // Catch-all fallthrough
            _ => println!(
                "Opcode {opcode:02X} (aaa:{aaa:03b} bbb:{bbb:03b} cc:{cc:02b}) not implemented"
            ),
        }
    }
    // clear carry flag
    fn clc(&mut self) {
        self.registers.carry = false;
    }

    // clear decimal flag
    fn cld(&mut self) {
        self.registers.decimal = false;
    }

    // clear interrupt disable flag
    fn cli(&mut self) {
        self.registers.interrupt_disable = false;
    }

    // clear zero flag
    fn clv(&mut self) {
        self.registers.zero = false;
    }

    // set clear flag
    fn sec(&mut self) {
        self.registers.carry = true;
    }
    // set decimal flag to true
    fn sed(&mut self) {
        self.registers.decimal = true;
    }
    // set i flag to true
    fn sei(&mut self) {
        self.registers.interrupt_disable = true;
    }

    // do nothing
    fn nop(&mut self) {}

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
            let old_pc = self.registers.pc;
            let new_pc = (old_pc as i32).wrapping_add(offset as i32) as u16;

            self.registers.pc = new_pc;

            if (old_pc & 0xFF00) != (new_pc & 0xFF00) {
                self.tick();
            }
        }
    }

    // load implementation
    pub fn get_operand_address(&mut self, mode: AddressingModes, bus: &mut Bus) -> AddrResult {
        let mut page_crossed = false;

        let address = match mode {
            AddressingModes::Immediate => {
                let addr = self.registers.pc;
                self.registers.pc = self.registers.pc.wrapping_add(1);
                addr
            }

            AddressingModes::ZeroPage => self.fetch(bus) as u16,

            AddressingModes::ZeroPageX => {
                let base = self.fetch(bus);
                base.wrapping_add(self.registers.get_x()) as u16
            }

            AddressingModes::ZeroPageY => {
                let base = self.fetch(bus);
                base.wrapping_add(self.registers.get_y()) as u16
            }

            AddressingModes::Absolute => {
                let lower = self.fetch(bus);
                let upper = self.fetch(bus);
                ((upper as u16) << 8) | (lower as u16)
            }

            AddressingModes::AbsoluteX => {
                let lower = self.fetch(bus);
                let upper = self.fetch(bus);
                let base = ((upper as u16) << 8) | (lower as u16);
                let addr = base.wrapping_add(self.registers.get_x() as u16);

                page_crossed = (base & 0xFF00) != (addr & 0xFF00);
                addr
            }

            AddressingModes::AbsoluteY => {
                let lower = self.fetch(bus);
                let upper = self.fetch(bus);
                let base = ((upper as u16) << 8) | (lower as u16);
                let addr = base.wrapping_add(self.registers.get_y() as u16);

                page_crossed = (base & 0xFF00) != (addr & 0xFF00);
                addr
            }

            AddressingModes::XIndexedZeroPageIndirect => {
                let base = self.fetch(bus);
                let ptr = base.wrapping_add(self.registers.get_x());

                let lower = self.read_bus(bus, ptr as u16);
                let upper = self.read_bus(bus, ptr.wrapping_add(1) as u16);

                ((upper as u16) << 8) | (lower as u16)
            }

            AddressingModes::ZeroPageIndirectYPaged => {
                let ptr = self.fetch(bus);

                let lower = self.read_bus(bus, ptr as u16);
                let upper = self.read_bus(bus, ptr.wrapping_add(1) as u16);
                let base = ((upper as u16) << 8) | (lower as u16);

                let addr = base.wrapping_add(self.registers.get_y() as u16);
                page_crossed = (base & 0xFF00) != (addr & 0xFF00);
                addr
            }

            AddressingModes::Indirect => {
                let lower = self.fetch(bus);
                let upper = self.fetch(bus);
                let ptr = ((upper as u16) << 8) | (lower as u16);

                let lower_target = self.read_bus(bus, ptr);
                let upper_target = if (ptr & 0x00FF) == 0x00FF {
                    self.read_bus(bus, ptr & 0xFF00)
                } else {
                    self.read_bus(bus, ptr + 1)
                };

                ((upper_target as u16) << 8) | (lower_target as u16)
            }

            AddressingModes::NoneAddressing => 0,
        };

        AddrResult {
            address,
            page_crossed,
        }
    }

    fn load_into_register(&mut self, register: LoadRegisters, data: u8) {
        match register {
            LoadRegisters::A => self.registers.set_a(data),
            LoadRegisters::X => self.registers.set_x(data),
            LoadRegisters::Y => self.registers.set_y(data),
        }
    }

    fn ld(&mut self, addressing_mode: AddressingModes, register: LoadRegisters, bus: &mut Bus) {
        let res = self.get_operand_address(addressing_mode, bus);
        let data = self.read_bus(bus, res.address);

        // 6502 loads get an extra cycle penalty ONLY if a page boundary is crossed
        if res.page_crossed {
            self.tick();
        }

        self.load_into_register(register, data);
        self.set_zero(data);
        self.set_negative(data);
    }

    // store
    fn get_register_data(&self, register: LoadRegisters) -> u8 {
        match register {
            LoadRegisters::A => return self.registers.get_a(),
            LoadRegisters::X => return self.registers.get_x(),
            LoadRegisters::Y => return self.registers.get_y(),
        }
    }

    fn st(&mut self, addressing_mode: AddressingModes, register: LoadRegisters, bus: &mut Bus) {
        let data = self.get_register_data(register);
        let address = self.get_operand_address(addressing_mode, bus);
        self.write_bus(bus, address.address, data);
    }

    fn php(&mut self, _: &mut Bus) {}
    fn plp(&mut self, _: &mut Bus) {}
    fn pha(&mut self, _: &mut Bus) {}
    fn pla(&mut self, _: &mut Bus) {}
    fn dey(&mut self) {}
    fn tay(&mut self) {}
    fn iny(&mut self) {}
    fn inx(&mut self) {}
    fn tya(&mut self) {}

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
