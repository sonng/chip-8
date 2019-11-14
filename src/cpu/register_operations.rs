use super::CPU;

impl CPU {
    pub(super) fn copy(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[y];
    }

    pub(super) fn or(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[x] | self.registers[y]
    }

    pub(super) fn and(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[x] & self.registers[y]
    }

    pub (super) fn xor(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[x] ^ self.registers[y]
    }

    pub (super) fn add(&mut self, x: usize, y: usize) {
        let result = (self.registers[x] as u16) + 
            (self.registers[y] as u16);

        self.registers[x] = result as u8;
        self.registers[15] = if result > 255 { 1 } else { 0 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
