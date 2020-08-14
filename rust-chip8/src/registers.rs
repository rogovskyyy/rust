struct Register {
    v: [u8; 16], // The V[F] register should not be used by any program, as it is used as a flag by some instructions. 
    i: u16, // generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    dt: u8, // delay timer 
    st: u8, // sound timer
    pc: u16, // program counter
    sp: u8,  // stack pointer
    stack: [u16; 16] // used to store the address that the interpreter shoud return to when finished with a subroutine
}

impl Register {

}
