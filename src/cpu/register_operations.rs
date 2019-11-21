use super::CPU;

impl CPU {
    pub(super) fn store_register(&mut self, x: usize, value: u8) {
        self.registers[x] = value;
    }

    pub(super) fn add_register(&mut self, x: usize, value: u8) {
        self.registers[x] = self.registers[x] + value;
    }

    pub(super) fn copy(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[y];
    }

    pub(super) fn or(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[x] | self.registers[y]
    }

    pub(super) fn and(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[x] & self.registers[y]
    }

    pub(super) fn xor(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[x] ^ self.registers[y]
    }

    pub(super) fn add(&mut self, x: usize, y: usize) {
        let result = (self.registers[x] as u16) + 
            (self.registers[y] as u16);

        self.registers[x] = result as u8;
        self.registers[15] = if result > 255 { 1 } else { 0 };
    }

    pub(super) fn sub(&mut self, x: usize, y: usize) {
        self.registers[15] = if self.registers[x] > self.registers[y] { 1 } else { 0 };
        self.registers[x] = ((self.registers[x] as i16) - 
            (self.registers[y] as i16)) as u8;
    }

    pub(super) fn subn(&mut self, x: usize, y: usize) {
        self.registers[15] = if self.registers[y] > self.registers[x] { 1 } else { 0 };
        self.registers[x] = ((self.registers[y] as i16) -
            (self.registers[x] as i16)) as u8;
    }

    pub(super) fn shift_right(&mut self, x: usize, y: usize) {
        self.registers[15] = self.registers[y] & 1;
        self.registers[x] = self.registers[y] >> 1;
    }

    pub(super) fn shift_left(&mut self, x: usize, y: usize) {
        self.registers[15] = (self.registers[y] & 0b10000000) >> 7;
        self.registers[x] = self.registers[y] << 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_register() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        test[0] = 0x60 as u8; test[1] = 0x21 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 0);

        chip8.run();
        assert_eq!(chip8.registers[0], 0x21 as u8);
    }

    #[test]
    fn test_add_register() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 2;
        test[0] = 0x70 as u8; test[1] = 0x05 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 2);

        chip8.run();
        assert_eq!(chip8.registers[0], 7);
    }

    #[test]
    fn test_copy_register() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[1] = 5;
        
        test[0] = 0x80 as u8; test[1] = 0x10 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 0);
        assert_eq!(chip8.registers[1], 5);

        chip8.run();
        assert_eq!(chip8.registers[0], 5);
        assert_eq!(chip8.registers[1], 5);
    }

    #[test]
    fn test_or_register() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 0b01010101 as u8;
        chip8.registers[1] = 0b10101010 as u8;

        test[0] = 0x80 as u8; test[1] = 0x11 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 0b01010101 as u8);
        assert_eq!(chip8.registers[1], 0b10101010 as u8);

        chip8.run();
        assert_eq!(chip8.registers[0], 0b11111111 as u8);
        assert_eq!(chip8.registers[1], 0b10101010 as u8);
    }

    #[test]
    fn test_and_register() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 0b01010101 as u8;
        chip8.registers[1] = 0b10101010 as u8;

        test[0] = 0x80 as u8; test[1] = 0x12 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 0b01010101 as u8);
        assert_eq!(chip8.registers[1], 0b10101010 as u8);

        chip8.run();
        assert_eq!(chip8.registers[0], 0b00000000 as u8);
        assert_eq!(chip8.registers[1], 0b10101010 as u8);
    }

    #[test]
    fn test_xor_register() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 0b01101001 as u8;
        chip8.registers[1] = 0b11110000 as u8;

        test[0] = 0x80 as u8; test[1] = 0x13 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 0b01101001 as u8);
        assert_eq!(chip8.registers[1], 0b11110000 as u8);

        chip8.run();
        assert_eq!(chip8.registers[0], 0b10011001 as u8);
        assert_eq!(chip8.registers[1], 0b11110000 as u8);
    }

    #[test]
    fn test_add_register_no_carry() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 5;
        chip8.registers[1] = 4;

        test[0] = 0x80 as u8; test[1] = 0x14 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 5);
        assert_eq!(chip8.registers[1], 4);
        assert_eq!(chip8.registers[15], 0);

        chip8.run();
        assert_eq!(chip8.registers[0], 9);
        assert_eq!(chip8.registers[1], 4);
        assert_eq!(chip8.registers[15], 0);
    }

    #[test]
    fn test_add_register_with_carry() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 255;
        chip8.registers[1] = 1;

        test[0] = 0x80 as u8; test[1] = 0x14 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 255);
        assert_eq!(chip8.registers[1], 1);
        assert_eq!(chip8.registers[15], 0);

        chip8.run();
        assert_eq!(chip8.registers[0], 0);
        assert_eq!(chip8.registers[1], 1);
        assert_eq!(chip8.registers[15], 1);
    }

    #[test]
    fn test_sub_register_no_carry() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 255;
        chip8.registers[1] = 1;

        test[0] = 0x80 as u8; test[1] = 0x15 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 255);
        assert_eq!(chip8.registers[1], 1);
        assert_eq!(chip8.registers[15], 0);

        chip8.run();
        assert_eq!(chip8.registers[0], 254);
        assert_eq!(chip8.registers[1], 1);
        assert_eq!(chip8.registers[15], 1);
    }

    #[test]
    fn test_sub_register_with_carry() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 0;
        chip8.registers[1] = 1;

        test[0] = 0x80 as u8; test[1] = 0x15 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 0);
        assert_eq!(chip8.registers[1], 1);
        assert_eq!(chip8.registers[15], 0);

        chip8.run();
        assert_eq!(chip8.registers[0], 255);
        assert_eq!(chip8.registers[1], 1);
        assert_eq!(chip8.registers[15], 0);
    }

    #[test]
    fn test_shift_right_zero_least_significant_bit() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 0b10011001 as u8;
        chip8.registers[1] = 0b10110110 as u8;

        test[0] = 0x80 as u8; test[1] = 0x16 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 0b10011001);
        assert_eq!(chip8.registers[1], 0b10110110);
        assert_eq!(chip8.registers[15], 0);

        chip8.run();
        assert_eq!(chip8.registers[0], 0b01011011);
        assert_eq!(chip8.registers[1], 0b10110110);
        assert_eq!(chip8.registers[15], 0);
    }

    #[test]
    fn test_shift_right_one_least_significant_bit() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 0b10011001 as u8;
        chip8.registers[1] = 0b10110111 as u8;

        test[0] = 0x80 as u8; test[1] = 0x16 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 0b10011001);
        assert_eq!(chip8.registers[1], 0b10110111);
        assert_eq!(chip8.registers[15], 0);

        chip8.run();
        assert_eq!(chip8.registers[0], 0b01011011);
        assert_eq!(chip8.registers[1], 0b10110111);
        assert_eq!(chip8.registers[15], 1);
    }

    #[test]
    fn test_subn_register_no_carry() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 100;
        chip8.registers[1] = 105;

        test[0] = 0x80 as u8; test[1] = 0x17 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 100);
        assert_eq!(chip8.registers[1], 105);
        assert_eq!(chip8.registers[15], 0);

        chip8.run();
        assert_eq!(chip8.registers[0], 5);
        assert_eq!(chip8.registers[1], 105);
        assert_eq!(chip8.registers[15], 1);
    }

    #[test]
    fn test_subn_register_with_carry() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 1;
        chip8.registers[1] = 0;

        test[0] = 0x80 as u8; test[1] = 0x17 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 1);
        assert_eq!(chip8.registers[1], 0);
        assert_eq!(chip8.registers[15], 0);

        chip8.run();
        assert_eq!(chip8.registers[0], 255);
        assert_eq!(chip8.registers[1], 0);
        assert_eq!(chip8.registers[15], 0);
    }

    #[test]
    fn test_shift_left_one_most_significant_bit() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 0b10011101;
        chip8.registers[1] = 0b11011100;

        test[0] = 0x80 as u8; test[1] = 0x1E as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 0b10011101);
        assert_eq!(chip8.registers[1], 0b11011100);
        assert_eq!(chip8.registers[15], 0);

        chip8.run();
        assert_eq!(chip8.registers[0], 0b10111000);
        assert_eq!(chip8.registers[1], 0b11011100);
        assert_eq!(chip8.registers[15], 1);
    }

    #[test]
    fn test_shift_left_zero_most_significant_bit() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 0b10011101;
        chip8.registers[1] = 0b01011101;

        test[0] = 0x80 as u8; test[1] = 0x1E as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 0b10011101);
        assert_eq!(chip8.registers[1], 0b01011101);
        assert_eq!(chip8.registers[15], 0);

        chip8.run();
        assert_eq!(chip8.registers[0], 0b10111010);
        assert_eq!(chip8.registers[1], 0b01011101);
        assert_eq!(chip8.registers[15], 0);
    }
}
