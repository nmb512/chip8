

fn assemble_address(n0: u8, n1: u8, n2: u8) -> u16 {
    (n0 as u16 & 0xf) | ((n1 as u16 & 0xf) << 4) | ((n2 as u16 & 0xf) << 8)
}

fn assemble_byte(k0: u8, k1: u8) -> u8 {
    (k0 & 0xf) | ((k1 & 0xf) << 4)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /// ### Clear Display
    /// Clear the Chip-8 display
    /// #### Assembly Syntax
    /// `CLS`
    Cls,
    /// ### Return from Subroutine
    /// Pop an address from the stack and store it in the program counter.
    /// #### Assembly Syntax
    /// `RET`
    Ret,
    /// ### Jump (immediate)
    /// Set PC = addr.
    /// #### Assembly Syntax
    /// `JP     addr`
    JpImm(u16),
    /// ### Call Subroutine
    /// Push the current program counter onto the stack. Then, set the program counter
    /// to a new value encoded as an immediate in the instruction.
    /// #### Assembly Syntax
    /// `CALL   addr`
    Call(u16),
    /// ### Skip if Equal (immediate)
    /// Skip the next instruction if Vx == byte.
    /// #### Assembly Syntax
    /// `SE     Vx, byte`
    SeImm(u8, u8),
    /// ### Skip if Not Equal (immediate)
    /// Skip the next instruction if Vx != byte.
    /// #### Assembly Syntax
    /// `SNE    Vx, byte`
    SneImm(u8, u8),
    /// ### Skip if Equal (register)
    /// Skip the next instruction if Vx != Vy.
    /// #### Assembly Syntax
    /// `SE     Vx, Vy`
    SeReg(u8, u8),
    /// ### Load (immediate)
    /// Set Vx = byte.
    /// #### Assembly Syntax
    /// `LD     Vx, byte`
    LdImm(u8, u8),
    /// ### Add (immediate)
    /// Set Vx = Vx + byte.
    /// #### Assembly Syntax
    /// `ADD    Vx, byte`
    AddImm(u8, u8),
    /// ### Load (register)
    /// Set Vx = Vy.
    /// #### Assembly Syntax
    /// `Vx = Vy`
    LdReg(u8, u8),
    /// ### Bitwise OR (register)
    /// Set Vx = Vx | Vy. 
    /// #### Assembly Syntax
    /// `OR     Vx, Vy`
    OrReg(u8, u8),
    /// ### Bitwise AND (register)
    /// Set Vx = Vx & Vy.
    /// #### Assembly Syntax
    /// `AND    Vx, Vy`
    AndReg(u8, u8),
    /// ### Bitwise XOR (register)
    /// Set Vx = Vx ^ Vy.
    /// #### Assembly Syntax
    /// `XOR    Vx, Vy`
    XorReg(u8, u8),
    /// ### Add (register)
    /// Set Vx = Vx + Vy. VF = carry.
    /// #### Assembly Syntax
    /// `ADD    Vx, Vy`
    AddReg(u8, u8),
    /// ### Subtract (register)
    /// Set Vx = Vx - Vy. VF = NOT borrow.
    /// #### Assembly Syntax
    /// `SUB    Vx, Vy`
    SubReg(u8, u8),
    /// ### Shift Right
    /// Set Vx = Vy >> 1. VF = carry.
    /// #### Assembly Syntax
    /// `SHR    Vx {, Vy}`
    Shr(u8, u8),
    /// ### Subtract (reversed)
    /// Set Vx = Vy - Vx. VF = NOT borrow.
    /// #### Assembly Syntax
    /// `SUBN   Vx, Vy`
    Subn(u8, u8),
    /// ### Shift Left
    /// Set Vx = Vx << 1. VF = carry.
    /// #### Assembly Syntax
    /// `SHL    Vx {, Vy}`
    Shl(u8, u8),
    /// ### Skip if Not Equal (register)
    /// Skip the next instruction if Vx != Vy.
    /// #### Assembly Syntax
    /// `SNE    Vx, Vy`
    SneReg(u8, u8),
    /// ### Load (index)
    /// Set I = addr.
    /// #### Assembly Syntax
    /// `LD     I, addr`
    LdI(u16),
    /// ### Jump (register)
    /// Set the program counter to addr + V0.
    /// #### Assembly Syntax
    /// `JP     V0, addr`
    JpReg(u16),
    /// ### Random Number Generation
    /// Set Vx = random & byte.
    /// #### Assembly Syntax
    /// `RND    Vx, byte`
    Rnd(u8, u8),
    /// ### Draw Sprite
    /// Display n-byte sprite starting at memory location I at (Vx, Vy). VF = collision.
    /// #### Assembly Syntax
    /// `DRW    Vx, Vy, n`
    Drw(u8, u8, u8),
    /// ### Skip if Key Pressed
    /// Skip the next instruction if key with value of Vx is pressed.
    /// #### Assembly Syntax
    /// `SKP    Vx`
    Skp(u8),
    /// ### Skip if Key Not Pressed
    /// Skip the next instruction if key with value of Vx is NOT pressed.
    /// #### Assembly Syntax
    /// `SKNP   Vx`
    Sknp(u8),
    /// ### Load (register, delay timer)
    /// Set Vx = delay timer.
    /// #### Assembly Syntax
    /// `LD     Vx, DT`
    LdRegDt(u8),
    /// ### Load (register, key)
    /// Wait for a key press, then store the value of the pressed key in Vx.
    /// #### Assembly Syntax
    /// `LD     Vx, K`
    LdRegK(u8),
    /// ### Load (delay timer, register)
    /// Set delay timer = Vx.
    /// #### Assembly Syntax
    /// `LD     DT, Vx`
    LdDtReg(u8),
    /// ### Load (sound timer)
    /// Set sound timer = Vx.
    /// #### Assembly Syntax
    /// `LD     ST, Vx`
    LdStReg(u8),
    /// ### Add (index)
    /// Set I = I + Vx.
    /// #### Assembly Syntax
    /// `ADD    I, Vx`
    AddI(u8),
    /// ### Load (font character)
    /// Set I = location of sprite for digit Vx.
    /// #### Assembly Syntax
    /// `LD     F, Vx`
    LdF(u8),
    /// ### Load (binary coded decimal)
    /// Store BCD representation of Vx in memory locations I, I+1, I+2.
    /// #### Assembly Syntax
    /// `LD     B, Vx`
    LdB(u8),
    /// ### Load (memory, register)
    /// Store registers V0 through Vx in memory starting at location I.
    /// #### Assembly Syntax
    /// `LD     [I], Vx`
    LdMemReg(u8),
    /// ### Load (register, memory)
    /// Read registers V0 through Vx from memory starting at location I.
    /// #### Assembly Syntax
    /// `LD     Vx, [I]`
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