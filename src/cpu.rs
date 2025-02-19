
#![allow(dead_code)]

use crate::instruction::Instruction;

const FONT_BASE_ADDRESS: u16 = 0x100;
const FONT_CHAR_SIZE: u16 = 5;          // Font sprites are 5 bytes long (8x5 pixels)

/// Stack of 16 16-bit values used for storing memory addresses.
pub struct Stack {
    bytes: [u16; 16],
    sp: usize,  // This is a usize so that it can be used to index a slice without type casting.
}

impl Stack {
    /// Create a new stack with all values initialized to zero.
    fn new() -> Self {
        Self {
            bytes: [0; 16],
            sp: 0,
        }
    }

    /// Push a value onto the stack and wrap the stack pointer if necessary.
    fn push(&mut self, data: u16) {
        self.bytes[self.sp] = data;
        self.sp += 1;

        // Wrap around stack pointer if it goes past the end of the stack
        if self.sp > (15) {
            self.sp = 0
        }
    }

    /// Pop a value from the stack and wrap the stack pointer if necessary.
    fn pop(&mut self) -> u16 {
        if self.sp == 0 {
            self.sp = 15;
        } else {
            self.sp -= 1;
        }

        self.bytes[self.sp]
    }

}

pub struct Cpu {
    /// Program counter (only 12 least significant bits used)
    pc: u16,

    /// Index register (only 12 least significant bits used)
    index: u16,

    /// Register file
    reg: [u8; 16],

    // TODO: Finish implementation of delay timer
    delay_timer: u8,

    // TODO: Finish implementation of sound timer
    sound_timer: u8,

    /// Random access memory
    memory: [u8; 4096],

    stack: Stack,

    // TODO: Implement display
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0x200,  // Most Chip-8 programs start at this address
            index: 0,
            reg: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; 4096],
            stack: Stack::new(),
        }
    }

    pub fn cycle(&mut self) {
        // Load instruction word from memory
        let instr_lo = self.memory[self.pc as usize];
        let instr_hi = self.memory[self.pc as usize + 1];
        let instr_word = ((instr_hi as u16) << 8) | instr_lo as u16;

        // Decode instruction word
        let instr = Instruction::decode(instr_word);

        // Increment program counter
        self.pc += 2;

        use Instruction::*;
        match instr {
            Cls => {
                // TODO: Implement display
                todo!();
            },
            Ret => {
                self.pc = self.stack.pop();
            },
            JpImm(addr) => {
                self.pc = addr;
            },
            Call(addr) => {
                self.stack.push(self.pc);
                self.pc = addr;
            },
            SeImm(vx, imm) => {
                if self.reg[vx as usize] == imm {
                    self.pc += 2;
                }
            },
            SneImm(vx, imm) => {
                if self.reg[vx as usize] != imm {
                    self.pc += 2;
                }
            },
            SeReg(vx, vy) => {
                if self.reg[vx as usize] == self.reg[vy as usize] {
                    self.pc += 2;
                }
            },
            LdImm(vx, imm) => {
                self.reg[vx as usize] = imm;
            },
            AddImm(vx, imm) => {
                (self.reg[vx as usize], _) = self.reg[vx as usize].overflowing_add(imm);
            },
            LdReg(vx, vy) => {
                self.reg[vx as usize] = self.reg[vy as usize];
            },
            OrReg(vx, vy) => {
                self.reg[vx as usize] |= self.reg[vy as usize];
            },
            AndReg(vx, vy) => {
                self.reg[vx as usize] &= self.reg[vy as usize];
            },
            XorReg(vx, vy) => {
                self.reg[vx as usize] ^= self.reg[vy as usize];
            },
            AddReg(vx, vy) => {
                let carry;
                (self.reg[vx as usize], carry) = self.reg[vx as usize].overflowing_add(self.reg[vy as usize]);
                // Set flag register based on carry
                self.reg[0xf] = if carry { 1 } else { 0 };
            },
            SubReg(vx, vy) => {
                let borrow;
                (self.reg[vx as usize], borrow) = self.reg[vx as usize].overflowing_sub(self.reg[vy as usize]);
                // Set flag register based on borrow
                self.reg[0xf] = if !borrow { 1 } else { 0 };
            },
            Shr(vx, _vy) => {
                let carry;
                (self.reg[vx as usize], carry) = self.reg[vx as usize].overflowing_shr(1);
                // Set flag register based on carry
                self.reg[0xf] = if carry { 1 } else { 0 };
            },
            Subn(vx, vy) => {
                let borrow;
                (self.reg[vx as usize], borrow) = self.reg[vy as usize].overflowing_sub(self.reg[vx as usize]);
                // Set flag register based on borrow
                self.reg[0xf] = if !borrow { 1 } else { 0 };
            },
            Shl(vx, _vy) => {
                let carry;
                (self.reg[vx as usize], carry) = self.reg[vx as usize].overflowing_shl(1);
                self.reg[0xf] = if carry { 1 } else { 0 };
            },
            SneReg(vx, vy) => {
                if self.reg[vx as usize] != self.reg[vy as usize] {
                    self.pc += 2;
                }
            },
            LdI(addr) => {
                self.index = addr;
            },
            JpReg(addr) => {
                self.pc = addr + self.reg[0] as u16;
            },
            Rnd(_vx, _imm) => {
                // TODO: Implement random number generation
                todo!();
            },
            Drw(_vx, _vy, _n) => {
                // TODO: Implement draw functionality
                todo!();
            },
            Skp(_vx) => {
                // TODO: Implement keypress detection
                todo!();
            },
            Sknp(_vx) => {
                // TODO: Implement keypress detection
                todo!();
            },
            LdRegDt(vx) => {
                self.reg[vx as usize] = self.delay_timer;
            },
            LdRegK(_vx) => {
                // TODO: implement keypress detection
                todo!()
            },
            LdDtReg(vx) => {
                self.delay_timer = self.reg[vx as usize]
            },
            LdStReg(vx) => {
                self.sound_timer = self.reg[vx as usize]
            },
            AddI(vx) => {
                self.index = self.index.wrapping_add(self.reg[vx as usize] as u16);
            },
            LdF(vx) => {
                self.index = FONT_BASE_ADDRESS + (vx as u16 * FONT_CHAR_SIZE);
            },
            LdB(vx) => {
                todo!()
            },
            LdMemReg(vx) => {
                for i in 0..=vx as usize {
                    self.memory[self.index as usize + i] = self.reg[i]
                }
            },
            LdRegMem(vx) =>{
                for i in 0..=vx as usize {
                    self.reg[i] = self.memory[self.index as usize + i]
                }
            },

            #[allow(unreachable_patterns)]
            _ => panic!("unimplemented instruction: {instr:?}"),
        }
    }
}