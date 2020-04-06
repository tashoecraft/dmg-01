pub enum Instruction {
    ADD(ArthimeticTarget),
    ADC(ArthimeticTarget),
    ADDHL(ADDHLTarget),
    SUB(ArthimeticTarget),
    SBC(ArthimeticTarget)
}

pub enum ArthimeticTarget {
    A, B, C, D, E, H, L,
}

pub enum ADDHLTarget {
    BC, DE, HL, SP
}
