use crate::memory::*;
use crate::registers::*;
use crate::display::*;
use crate::keyboard::*;
use rand::Rng;

pub struct Processor;
enum Action {
        Next,
        Skip,
        Jump(u16)
}

impl Action {
    pub fn skip_if(condition: bool) -> Action {
        if condition {
            Action::Skip
        } else {
            Action::Next
        }
    }
}

impl Processor  {
    // DICTIONARY
    //nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
    //n or nibble - A 4-bit value, the lowest 4 bits of the instruction
    //x - A 4-bit value, the lower 4 bits of the high byte of the instruction
    //y - A 4-bit value, the upper 4 bits of the low byte of the instruction
    //kk or byte - An 8-bit value, the lowest 8 bits of the instruction
    //
    //
    // BITWISE OPERATORS CHEAT SHEET
    //  !	Macro expansion	
    //  !	!expr	Bitwise or logical complement	Not
    //  !=	var != expr	Nonequality comparison	PartialEq
    //  %	expr % expr	Arithmetic remainder	Rem
    //  %=	var %= expr	Arithmetic remainder and assignment	RemAssign
    //  &	&expr, &mut expr	Borrow	
    //  &	&type, &mut type, &'a type, &'a mut type	Borrowed pointer type	
    //  &	expr & expr	Bitwise AND	BitAnd
    //  &=	var &= expr	Bitwise AND and assignment	BitAndAssign
    //  &&	expr && expr	Short-circuiting logical AND	
    //  *	expr * expr	Arithmetic multiplication	Mul
    //  *=	var *= expr	Arithmetic multiplication and assignment	MulAssign
    //  *	*expr	Dereference	
    //  *	*const type, *mut type	Raw pointer	
    //  +	trait + trait, 'a + trait	Compound type constraint	
    //  +	expr + expr	Arithmetic addition	Add
    //  +=	var += expr	Arithmetic addition and assignment	AddAssign
    //  ,	expr, expr	Argument and element separator	
    //  -	- expr	Arithmetic negation	Neg
    //  -	expr - expr	Arithmetic subtraction	Sub
    //  -=	var -= expr	Arithmetic subtraction and assignment	SubAssign
    //  ->	fn(...) -> type, |...| -> type	Function and closure return type	
    //  .	expr.ident	Member access	
    //  ..	.., expr.., ..expr, expr..expr	Right-exclusive range literal	
    //  ..=	..=expr, expr..=expr	Right-inclusive range literal	
    //  ..	..expr	Struct literal update syntax	
    //  ..	variant(x, ..), struct_type { x, .. }	“And the rest” pattern binding	
    //  ...	expr...expr	In a pattern: inclusive range pattern	
    //  /	expr / expr	Arithmetic division	Div
    //  /=	var /= expr	Arithmetic division and assignment	DivAssign
    //  :	pat: type, ident: type	Constraints	
    //  :	ident: expr	Struct field initializer	
    //  :	'a: loop {...}	Loop label	
    //  ;	expr;	Statement and item terminator	
    //  ;	[...; len]	Part of fixed-size array syntax	
    //  <<	expr << expr	Left-shift	Shl
    //  <<=	var <<= expr	Left-shift and assignment	ShlAssign
    //  <	expr < expr	Less than comparison	PartialOrd
    //  <=	expr <= expr	Less than or equal to comparison	PartialOrd
    //  =	var = expr, ident = type	Assignment/equivalence	
    //  ==	expr == expr	Equality comparison	PartialEq
    //  =>	pat => expr	Part of match arm syntax	
    //  >	expr > expr	Greater than comparison	PartialOrd
    //  >=	expr >= expr	Greater than or equal to comparison	PartialOrd
    //  >>	expr >> expr	Right-shift	Shr
    //  >>=	var >>= expr	Right-shift and assignment	ShrAssign
    //  @	ident @ pat	Pattern binding	
    //  ^	expr ^ expr	Bitwise exclusive OR	BitXor
    //  ^=	var ^= expr	Bitwise exclusive OR and assignment	BitXorAssign
    //  |	pat | pat	Pattern alternatives	
    //  |	expr | expr	Bitwise OR	BitOr
    //  |=	var |= expr	Bitwise OR and assignment	BitOrAssign
    //  ||	expr || expr	Short-circuiting logical OR	
    //  ?	expr?	Error propagation	

    pub fn create() {
        let mut m = Memory::new();
        let mut r = Register::new();
        let mut d = Display::new();
        let mut k = Keyboard::new();
    }

    //Clear the display.
    fn i00e0(mut a: Display) -> Action {
        for j in 0..32 {
            for i in 0..64 {
                a.vram[i][j] = 0x0;
            }
        }
        Action::Next
    }

    //Return from a subroutine.
    //The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    fn i00ee(mut r: Register) -> Action {
        r.sp -= 1;
        Action::Jump(r.stack[r.sp as usize])
    }

    //Jump to location nnn.
    //The interpreter sets the program counter to nnn.
    fn i1nnn(mut r: Register, nnn: u16) -> Action {
        r.pc = nnn;
        Action::Jump(nnn)
    }

    //Call subroutine at nnn.
    //The interpreter increments the stack pointer, then puts the current PC on the top of the stack. 
    //The PC is then set to nnn.
    fn i2nnn(mut r: Register, nnn: u16) -> Action {
        r.stack[r.sp as usize] = r.pc;
        r.sp += 1;
        Action::Jump(nnn)

    }

    //skip next instruction if Vx = kk.
    //The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn i3xkk(r: Register, x: u8, kk: u8) -> Action {
        Action::skip_if(r.v[x as usize] == kk)
    }

    //Skip next instruction if Vx != kk.
    //The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn i4xkk(r: Register, x: u8, kk: u8) -> Action {
        Action::skip_if(r.v[x as usize] != kk)
    }

    //Skip next instruction if Vx = Vy.
    //The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn i5xy0(r: Register, x: u8, y: u8) -> Action {
        Action::skip_if(r.v[x as usize] == r.v[y as usize])
    }

    //Set Vx = kk.
    //The interpreter puts the value kk into register Vx.
    fn i6xkk(mut r: Register, x: u8, kk: u8) -> Action {
        r.v[x as usize] = kk;
        Action::Next
    }

    //Set Vx = Vx + kk.
    //Adds the value kk to the value of register Vx, then stores the result in Vx. 
    fn i7xkk(mut r: Register, x: u8, kk: u8) -> Action {
        r.v[x as usize] = (r.v[x as usize] + kk) as u8;
        Action::Next
    }

    //Set Vx = Vy.
    //Stores the value of register Vy in register Vx.
    fn i8xy0(mut r: Register, x: u8, y: u8) -> Action {
        r.v[x as usize] = r.v[y as usize];
        Action::Next
    }

    //Set Vx = Vx OR Vy.
    //Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. 
    //A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then 
    //the same bit in the result is also 1. Otherwise, it is 0.
    fn i8xy1(mut r: Register, x: u8, y: u8) -> Action {
        r.v[x as usize] |= r.v[y as usize];
        Action::Next
    }

    //Set Vx = Vx AND Vy.
    //Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. 
    //A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, 
    //then the same bit in the result is also 1. Otherwise, it is 0.
    fn i8xy2(mut r: Register, x: u8, y: u8) -> Action {
        r.v[x as usize] &= r.v[y as usize];
        Action::Next
    }

    //Set Vx = Vx XOR Vy.
    //Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    //An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, 
    //then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn i8xy3(mut r: Register, x: u8, y: u8) -> Action {
        r.v[x as usize] ^= r.v[y as usize];
        Action::Next
    }

    //Set Vx = Vx + Vy, set VF = carry.
    //The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) 
    //VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn i8xy4(mut r: Register, x: usize, y: usize) -> Action {
        let temp: u16 = (r.v[x] + r.v[y]).into();
        r.v[0xf] = if temp > 255 {
            1
        } else {
            0
        };
        Action::Next
    }

    //Set Vx = Vx - Vy, set VF = NOT borrow.
    //If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn i8xy5(mut r: Register, x: u8, y: u8) -> Action {
        r.v[0xf] = if r.v[x as usize] > r.v[y as usize] {
            1
        } else {
            0
        };
        r.v[x as usize] = r.v[y as usize] - r.v[x as usize];
        Action::Next
    }

    //Set Vx = Vx SHR 1.
    //If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn i8x06(mut r: Register, x: u8, y: u8) -> Action {
        r.v[0xf] = r.v[x as usize] & 1;
        r.v[x as usize] >>= 1;
        Action::Next
    }

    //Set Vx = Vy - Vx, set VF = NOT borrow.
    //If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn i8xy7(mut r: Register, x: u8, y: u8) -> Action {
        r.v[0xf] = if r.v[y as usize] > r.v[x as usize] {
            1
        } else {
            0
        };
        r.v[x as usize] = r.v[x as usize] - r.v[y as usize];
        Action::Next
    }

    //Set Vx = Vx SHL 1.
    //If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    fn i8x0E(mut r: Register, x: u8) -> Action {
        r.v[0x0f] = (r.v[x as usize] & 0b10000000) >> 7;
        r.v[x as usize] <<= 1;
        Action::Next
    }

    //Skip next instruction if Vx != Vy.
    //The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    fn i9xy0(r: Register, x: u8, y: u8) -> Action {
        Action::skip_if(r.v[x as usize] != r.v[y as usize])
    }

    //Set I = nnn.
    //The value of register I is set to nnn.
    fn iAnnn(mut r: Register, nnn: u16) -> Action {
        r.i = nnn;
        Action::Next
    }

    //Jump to location nnn + V0.
    //The program counter is set to nnn plus the value of V0.
    fn iBnnn(r: Register, nnn: u16) -> Action {
        let jumper: u16 = r.v[0] as u16 + nnn;
        Action::Jump(jumper)
    }

    //Set Vx = random byte AND kk.
    //The interpreter generates a random number from 0 to 255, 
    //which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    fn iCxkk(mut r: Register, x: u8, kk: u8) -> Action {
        let mut random = rand::thread_rng();
        random.gen_range(0x0, 0xff);
        r.v[x as usize] = random.gen_range(0x0, 0xff) & kk;
        Action::Next
    }

    //Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    //The interpreter reads n bytes from memory, starting at the address stored in I. 
    //These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). 
    //Sprites are XORed onto the existing screen. If this causes any pixels to be erased, 
    //VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the 
    //coordinates of the display, it wraps around to the opposite side of the screen. 
    //See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    fn iDxyn() {}

    //Skip next instruction if key with the value of Vx is pressed.
    //Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, 
    //PC is increased by 2.
    fn iEx9E() {}

    //Skip next instruction if key with the value of Vx is not pressed.
    //Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    fn iExA1() {}

    //Set Vx = delay timer value.
    //The value of DT is placed into Vx.
    fn iFx07() {}

    //Wait for a key press, store the value of the key in Vx.
    //All execution stops until a key is pressed, then the value of that key is stored in Vx.
    fn iFx0A() {}

    //Set delay timer = Vx.
    //DT is set equal to the value of Vx.
    fn iFx15() {}

    //Set sound timer = Vx.
    //ST is set equal to the value of Vx.
    fn iFx18() {}

    //Set I = I + Vx.
    //The values of I and Vx are added, and the results are stored in I
    fn iFx1E() {}

    //Set I = location of sprite for digit Vx.
    //The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. 
    //See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    fn iFx29() {}

    //Store BCD representation of Vx in memory locations I, I+1, and I+2.
    //The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, 
    //the tens digit at location I+1, and the ones digit at location I+2.
    fn iFx33() {}

    //Store registers V0 through Vx in memory starting at location I.
    //The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    fn iFx55() {}

    //Read registers V0 through Vx from memory starting at location I.
    //The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn iFx65() {}
}