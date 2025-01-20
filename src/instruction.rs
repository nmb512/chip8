

fn assemble_address(n0: u8, n1: u8, n2: u8) -> u16 {
    (n0 as u16 & 0xf) | ((n1 as u16 & 0xf) << 4) | ((n2 as u16 & 0xf) << 8)
}

fn assemble_byte(k0: u8, k1: u8) -> u8 {
    (k0 & 0xf) | ((k1 & 0xf) << 4)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Cls,
    Ret,
    JpImm(u16),
    Call(u16),
    SeImm(u8, u8),
    SneImm(u8, u8),
    SeReg(u8, u8),
    LdImm(u8, u8),
    AddImm(u8, u8),
    LdReg(u8, u8),
    OrReg(u8, u8),
    AndReg(u8, u8),
    XorReg(u8, u8),
    AddReg(u8, u8),
    SubReg(u8, u8),
    Shr(u8, u8),
    Subn(u8, u8),
    Shl(u8, u8),
    SneReg(u8, u8),
    LdI(u16),
    JpReg(u16),
    Rnd(u8, u8),
    Drw(u8, u8, u8),
    Skp(u8),
    Sknp(u8),
    LdRegDt(u8),
    LdRegK(u8),
    LdDtReg(u8),
    LdStReg(u8),
    AddI(u8),
    LdF(u8),
    LdB(u8),
    LdMemReg(u8),
    LdRegMem(u8),
}

impl Instruction {
    pub fn decode(source: u16) -> Instruction {
        let n0 = ((source >> 0 ) & 0xf) as u8;
        let n1 = ((source >> 4 ) & 0xf) as u8;
        let n2 = ((source >> 8 ) & 0xf) as u8;
        let n3 = ((source >> 12) & 0xf) as u8;

        match (n3, n2, n1, n0) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::Cls,
            (0x0, 0x0, 0xE, 0xE) => Instruction::Ret,
            (0x1,  n2,  n1,  n0) => Instruction::JpImm(assemble_address(n0, n1, n2)),
            (0x2,  n2,  n1,  n0) => Instruction::Call(assemble_address(n0, n1, n2)),
            (0x3,   x,  k1,  k0) => Instruction::SeImm(x, assemble_byte(k0, k1)),
            (0x4,   x,  k1,  k0) => Instruction::SneImm(x, assemble_byte(k0, k1)),
            (0x5,   x,   y, 0x0) => Instruction::SeReg(x, y),
            (0x6,   x,  k1,  k0) => Instruction::LdImm(x, assemble_byte(k0, k1)),
            (0x7,   x,  k1,  k0) => Instruction::AddImm(x, assemble_byte(k0, k1)),
            (0x8,   x,   y, 0x0) => Instruction::LdReg(x, y),
            (0x8,   x,   y, 0x1) => Instruction::OrReg(x, y),
            (0x8,   x,   y, 0x2) => Instruction::AndReg(x, y),
            (0x8,   x,   y, 0x3) => Instruction::XorReg(x, y),
            (0x8,   x,   y, 0x4) => Instruction::AddReg(x, y),
            (0x8,   x,   y, 0x5) => Instruction::SubReg(x, y),
            (0x8,   x,   y, 0x6) => Instruction::Shr(x, y),
            (0x8,   x,   y, 0x7) => Instruction::Subn(x, y),
            (0x8,   x,   y, 0xE) => Instruction::Shl(x, y),
            (0x9,   x,   y, 0x0) => Instruction::SneReg(x, y),
            (0xA,  n2,  n1,  n0) => Instruction::LdI(assemble_address(n0, n1, n2)),
            (0xB,  n2,  n1,  n0) => Instruction::JpReg(assemble_address(n0, n1, n2)),
            (0xC,   x,  k1,  k0) => Instruction::Rnd(x, assemble_byte(k0, k1)),
            (0xD,   x,   y,   n) => Instruction::Drw(x, y, n),
            (0xE,   x, 0x9, 0xE) => Instruction::Skp(x),
            (0xE,   x, 0xA, 0x1) => Instruction::Sknp(x),
            (0xF,   x, 0x0, 0x7) => Instruction::LdRegDt(x),
            (0xF,   x, 0x0, 0xA) => Instruction::LdRegK(x),
            (0xF,   x, 0x1, 0x5) => Instruction::LdDtReg(x),
            (0xF,   x, 0x1, 0x8) => Instruction::LdStReg(x),
            (0xF,   x, 0x1, 0xE) => Instruction::AddI(x),
            (0xF,   x, 0x2, 0x9) => Instruction::LdF(x),
            (0xF,   x, 0x3, 0x3) => Instruction::LdB(x),
            (0xF,   x, 0x5, 0x5) => Instruction::LdMemReg(x),
            (0xF,   x, 0x6, 0x5) => Instruction::LdRegMem(x),

            _ => panic!("decoded invalid instruction: {source:#04x}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_test() {
        use Instruction::*;
        let decode_table: Vec<(u16, Instruction)> = vec![
            (0x00E0, Cls),
            (0x00EE, Ret),
            (0x10ff, JpImm(0x0ff)),
            (0x2fcc, Call(0xfcc)),
            (0x3381, SeImm(3, 0x81)),
            (0x4242, SneImm(2, 0x42)),
            (0x5a80, SeReg(0xa, 0x8)),
            (0x6555, LdImm(5, 0x55)),

            (0x8980, LdReg(9, 8)),
            (0xA123, LdI(0x123)),
        ];

        for (bytes, instr) in decode_table {
            assert_eq!(Instruction::decode(bytes), instr)
        }
    }
}