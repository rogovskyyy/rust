pub struct Display {
    pub vram: [[u8; 64]; 32]
}

impl Display {
    pub fn new() -> Self {
        Display {
            vram: [[0; 64]; 32]
        }
    }
}