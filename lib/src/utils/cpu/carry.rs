pub struct Carry;

impl Carry {
    pub fn add_from_u8(a: u8, b: u8) -> bool {
        ((a as u16 & 0xFF) + (b as u16 & 0xFF)) > 0xFF
    }

    pub fn add_from_u8_with_carry(a: u8, b: u8, carry: u8) -> bool {
        ((a as u16 & 0xFF) + (b as u16 & 0xFF)) + carry as u16 > 0xFF
    }

    pub fn sub_from_u8(a: u8, b: u8) -> bool {
        a < b
    }

    pub fn sub_from_u8_with_carry(a: u8, b: u8, carry: u8) -> bool {
        (a as u16) < ((b as u16) + (carry as u16))
    }

    pub fn add_from_u16(a: u16, b: u16) -> bool {
        a < b
    }

    pub fn add_from_i32(a: i32, b: i32) -> bool {
        ((a & 0xFF) + (b & 0xFF)) > 0xFF
    }
}
