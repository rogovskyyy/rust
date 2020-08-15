pub struct Register {
    pub v: [u8; 16], // V register | The V[F] register should not be used by any program, as it is used as a flag by some instructions. 
    pub i: u16, // I register | generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    pub dt: u8, // delay timer 
    pub st: u8, // sound timer
    pub pc: u16, // program counter
    pub sp: u8,  // stack pointer
    pub stack: [u16; 16] // used to store the address that the interpreter shoud return to when finished with a subroutine
}

impl Register {
    pub fn new() -> Self {
        Register {
            v: [0x0; 16],
            i:  0x0,
            dt: 0x0,
            st: 0x0,
            pc: 0x200,
            sp: 0x0,
            stack: [0x0; 16]
        }
    }
}
