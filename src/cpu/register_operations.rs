use super::CPU;

impl CPU {

    pub(super) fn copy(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[y];
    }

    pub(super) fn or(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[x] | self.registers[y]
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
}
