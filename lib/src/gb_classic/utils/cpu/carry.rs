pub struct Carry;

impl Carry {
    pub fn add_from_u8(a: u8, b: u8) -> bool {
        a.checked_add(b).is_none()
    }

    pub fn add_from_u8_with_carry(a: u8, b: u8, carry: u8) -> bool {
        a.checked_add(b)
            .and_then(|sum| sum.checked_add(carry))
            .is_none()
    }

    pub fn sub_from_u8(a: u8, b: u8) -> bool {
        a.checked_sub(b).is_none()
    }

    pub fn sub_from_u8_with_carry(a: u8, b: u8, carry: u8) -> bool {
        a.checked_sub(b)
            .and_then(|diff| diff.checked_sub(carry))
            .is_none()
    }

    pub fn add_from_u16(a: u16, b: u16) -> bool {
        a.checked_add(b).is_none()
    }

    pub fn add_from_i32(a: i32, b: i32) -> bool {
        let a = (a & 0xFF) as u8;
        let b = (b & 0xFF) as u8;

        a.checked_add(b).is_none()
    }
}

#[cfg(test)]
mod utils_carry_tests {
    use super::*;

    #[test]
    fn add_from_u8_overflow() {
        // a + b = 0x100
        let a = 0x80;
        let b = 0x80;

        assert_eq!(Carry::add_from_u8(a, b), true);
    }

    #[test]
    fn add_from_u8() {
        // a + b = 0xFF
        let a = 0x80;
        let b = 0x7F;

        assert_eq!(Carry::add_from_u8(a, b), false);
    }

    #[test]
    fn add_from_u8_with_carry_overflow() {
        // a + b + carry = 0x100
        let a = 0x80;
        let b = 0x7F;
        let carry = 0x01;

        assert_eq!(Carry::add_from_u8_with_carry(a, b, carry), true);
    }

    #[test]
    fn add_from_u8_with() {
        // a + b + carry = 0xFF
        let a = 0x80;
        let b = 0x7E;
        let carry = 0x01;

        assert_eq!(Carry::add_from_u8_with_carry(a, b, carry), false);
    }

    #[test]
    fn sub_from_u8_overflow() {
        let a = 0x0F;
        let b = 0x10;

        assert_eq!(Carry::sub_from_u8(a, b), true);
    }

    #[test]
    fn sub_from_u8() {
        let a = 0xFF;
        let b = 0xFE;

        assert_eq!(Carry::sub_from_u8(a, b), false);
    }

    #[test]
    fn sub_from_u8_with_carry_overflow() {
        let a = 0x0F;
        let b = 0x0F;
        let carry = 0x01;

        assert_eq!(Carry::sub_from_u8_with_carry(a, b, carry), true);
    }

    #[test]
    fn sub_from_u8_with_carry() {
        let a = 0x0F;
        let b = 0x0E;
        let carry = 0x01;

        assert_eq!(Carry::sub_from_u8_with_carry(a, b, carry), false);
    }
}
