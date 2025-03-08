pub struct HalfCarry;

impl HalfCarry {
    pub fn add_from_u8(a: u8, b: u8) -> bool {
        ((a & 0xF) + (b & 0xF)) > 0xF
    }

    pub fn add_from_u8_with_carry(a: u8, b: u8, carry: u8) -> bool {
        ((a & 0xF) + (b & 0xF) + carry) > 0xF
    }

    pub fn sub_from_u8(a: u8, b: u8) -> bool {
        (a & 0x0F) < (b & 0x0F)
    }

    pub fn sub_from_u8_with_carry(a: u8, b: u8, carry: u8) -> bool {
        (a & 0x0F) < ((b & 0x0F) + carry)
    }

    pub fn add_from_u16(a: u16, b: u16) -> bool {
        ((a & 0xFFF) + (b & 0xFFF)) > 0xFFF
    }

    pub fn add_from_i32(a: i32, b: i32) -> bool {
        ((a & 0xF) + (b & 0xF)) > 0xF
    }
}
