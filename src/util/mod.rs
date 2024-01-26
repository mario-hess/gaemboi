pub fn bit_enabled(byte: &u8, index: u8) -> bool {
    (1 << index) & byte > 0
}
