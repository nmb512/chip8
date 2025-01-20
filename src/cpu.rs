
#![allow(dead_code)]

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
        todo!();
    }
}