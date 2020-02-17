pub mod flags_register;
pub mod instruction;
pub mod registers;

use self::instruction::{
    Instruction, ArthimeticTarget, ADDHLTarget
};
use self::registers::Registers;

use self::flags_register::FlagsRegister;

pub struct CPU {
    pub registers: Registers
}
impl CPU {
    pub fn new() -> CPU {
        CPU {            
            registers: Registers::new()
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    // ADD A,n START
                    ArthimeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArthimeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArthimeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArthimeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArthimeticTarget::D => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArthimeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArthimeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => { /* Error for unsupported add target */}
                }
            }
            Instruction::ADDHL(target) => {
                match target {
                    ADDHLTarget::BC => {
                        let value = self.registers.get_bc();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                    }
                    ADDHLTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                    }
                    ADDHLTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                    }
                    ADDHLTarget::SP => {
                        // let value = self.registers.get_sp();
                        // let new_value = self.add_hl(value);
                        // self.registers.set_hl(new_value);
                    }
                    _ => { /* Error for unsupported addHL target */}
                }
            }
            _ => { /* TODO: support more instructions */ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry check if the lower nibble 00001111 will overflow
        // so if a is 00001111, then it is anded 00001111 -> 00001111 
        // added to value, which does the same thing
        // if greater than 0xF, then it ups half carry
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    fn add_hl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;

        self.registers.f.half_carry = (self.registers.get_hl() & 0xFF) + (value & 0xFF) > 0xF;
        new_value
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        new_value
    }
}

#[cfg(test)]
mod cpu_execute_tests {
    use super::*;
    #[test]
    fn cpu_execute_b2a() {
        let mut cpu = CPU::new();
        cpu.registers.b = 1;
        cpu.execute(Instruction::ADD(ArthimeticTarget::B));
        assert_eq!(cpu.registers.a, 1);
    }
    #[test]
    fn cpu_execute_a2a() {
        let mut cpu = CPU::new();
        cpu.registers.a = 1;
        cpu.execute(Instruction::ADD(ArthimeticTarget::A));
        assert_eq!(cpu.registers.a, 2);
    }
    #[test]
    fn cpu_execute_bc_2_hl() {
        let mut cpu = CPU::new();
        cpu.registers.set_bc(250);
        cpu.execute(Instruction::ADDHL(ADDHLTarget::BC));
        assert_eq!(cpu.registers.get_hl(), 250);
    }
    #[test]
    fn cpu_execute_de_2_hl() {
        let mut cpu = CPU::new();
        cpu.registers.a = 1;
        cpu.execute(Instruction::ADD(ArthimeticTarget::A));
        assert_eq!(cpu.registers.a, 2);
    }
}

#[cfg(test)]
mod cpu_add_tests {
    use std::u8::MAX;
    use super::*;
    #[test]
    fn cpu_add() {
        let mut cpu = CPU::new();
        cpu.registers.a = 1;
        cpu.registers.f.subtract = true;
        let new_value = cpu.add(5);
        assert_eq!(new_value, 6);
        assert_eq!(cpu.registers.f.subtract, false);
    }
    #[test]
    fn cpu_add_zero() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0;
        let new_value = cpu.add(0);
        assert_eq!(new_value, 0);
        assert_eq!(cpu.registers.f.zero, true);
    }
    #[test]
    fn cpu_add_carry_no_half_carry() {
        let mut cpu = CPU::new();
        cpu.registers.a = 240;
        let new_value = cpu.add(17);
        assert_eq!(new_value, 1);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
    }
    #[test]
    fn cpu_add_half_carry_carry() {
        let mut cpu = CPU::new();
        cpu.registers.a = MAX;
        let new_value = cpu.add(1);
        assert_eq!(new_value, 0);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);
    }
    #[test]
    fn cpu_add_half_carry_no_carry() {
        let mut cpu = CPU::new();
        cpu.registers.a = 15;
        let new_value = cpu.add(1);
        assert_eq!(new_value, 16);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);
    }
}

#[cfg(test)]
mod add_hl_tests {
    use super::*;
    use std::u8::MAX;
    #[test]
    fn cpu_add_hl() {
        let mut cpu = CPU::new();
        cpu.registers.set_hl(200);
        cpu.registers.f.subtract = true;
        let new_value = cpu.add_hl(1);
        assert_eq!(new_value, 201);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, true);
    }
    #[test]
    fn cpu_add_hl_no_half_carry() {
        let mut cpu = CPU::new();
        cpu.registers.set_hl(10);
        let new_value = cpu.add_hl(5);
        assert_eq!(new_value, 15);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, false);
    }
    #[test]
    fn cpu_add_hl_half_carry() {
        let mut cpu = CPU::new();
        cpu.registers.set_hl(0);
        let new_value = cpu.add_hl(16);
        assert_eq!(new_value, 16);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, true);
    }
    #[test]
    fn cpu_add_hl_carry_zero() {
        let mut cpu = CPU::new();
        cpu.registers.set_hl(65535);
        let new_value = cpu.add_hl(1);
        assert_eq!(new_value, 0);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, true);
    }
    #[test]
    fn cpu_add_hl_carry_no_half() {
        let mut cpu = CPU::new();
        cpu.registers.set_hl(65280);
        let new_value = cpu.add_hl(65280);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, false);
    }
}

#[cfg(test)]
mod sub_tests {
    use super::*;

}