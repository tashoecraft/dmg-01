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
                    // ADD A,n END
                    _ => { /* TODO: support more targets */}
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

        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cpu_execute() {
        let mut cpu = CPU::new();
        cpu.registers.b = 1;
        cpu.execute(Instruction::ADD(ArthimeticTarget::B));
        assert_eq!(cpu.registers.a, 1);
    }
}