pub enum Instruction {
    ADD(ArthimeticTarget),
}

pub enum ArthimeticTarget {
    A, B, C, D, E, H, L,
}

pub enum ADDHLTarget {
    BC, DE, HL, SP
}