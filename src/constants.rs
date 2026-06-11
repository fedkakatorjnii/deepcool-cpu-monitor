pub const LIGHT: u8 = 16;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Case {
    Temp = 19,
    Usage = 76,
    Load = 170,
    // Init = 255,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Mode {
    Solid = 0,
    // Blink = 2,
    // PartialBlink = 1,
}
