/*
 * 0xFF24 — NR50 (Master volume)
 * Bit 4-6: Left volume, Bit 0-2: Right volume
 * These specify the master volume, i.e. how much each outputvshould be scaled.
 * A value of 0 is treated as a volume of 1 (very quiet), and a value of 7 is
 * treated as a volume of 8 (no volume reduction). Importantly, the amplifier
 * never mutes a non-silent input.
 */
pub struct MasterVolume {
    right_volume: u8,
    right_vin: bool,
    left_volume: u8,
    left_vin: bool,
}

impl MasterVolume {
    pub fn new() -> Self {
        Self {
            right_volume: 0x07,
            right_vin: false,
            left_volume: 0x07,
            left_vin: false,
        }
    }

    pub fn get_master_volume(&self) -> u8 {
        let right_volume = self.right_volume;
        let vin_right = if self.right_vin { 0x08 } else { 0 };
        let left_volume = self.left_volume << 4;
        let vin_left = if self.left_vin { 0x80 } else { 0 };

        right_volume | vin_right | left_volume | vin_left
    }

    pub fn set_master_volume(&mut self, value: u8) {
        self.right_volume = value & 0x07;
        self.right_vin = value & 0x08 != 0;
        self.left_volume = (value & 0x70) >> 4;
        self.left_vin = value & 0x80 != 0;
    }

    pub fn get_left_volume(&self) -> &u8 {
        &self.left_volume
    }

    pub fn get_right_volume(&self) -> &u8 {
        &self.right_volume
    }

    pub fn reset(&mut self) {
        self.left_volume = 0;
        self.left_vin = false;
        self.right_volume = 0;
        self.right_vin = false;
    }
}
