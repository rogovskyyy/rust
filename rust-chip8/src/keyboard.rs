pub struct Keyboard {
    pub keys: [[bool; 4]; 4],
    pub pressed: u8
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [[false; 4]; 4],
            pressed: 0x0
        }
    }
}