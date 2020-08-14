struct Keyboard {
    keys: [[u8; 4]; 4],
    pressed: u8
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys[0][0] = 0x1,
            keys[0][1] = 0x2,
            keys[0][2] = 0x3,
            keys[0][3] = 0xC,
            keys[1][0] = 0x4,
            keys[1][1] = 0x5,
            keys[1][2] = 0x6,
            keys[1][3] = 0xD,
            keys[2][0] = 0x7,
            keys[2][1] = 0x8,
            keys[2][2] = 0x9,
            keys[2][3] = 0xE,
            keys[3][0] = 0xA,
            keys[3][1] = 0x0,
            keys[3][2] = 0xB,
            keys[3][3] = 0xF,
        }
    }
}