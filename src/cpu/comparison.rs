use super::CPU;

impl CPU {
    pub(super) fn skip_if_equal(&mut self, x: usize, value: u8) {
        if self.registers[x] == value {
            self.cur_pos += 2;
        }
    }

    pub(super) fn skip_if_not_equal(&mut self, x: usize, value: u8) {
        if self.registers[x] != value {
            self.cur_pos += 2;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_if_equal() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 5;
        chip8.registers[1] = 6;

        test[0] = 0x30 as u8; test[1] = 0x05 as u8;
        test[2] = 0x80 as u8; test[3] = 0x14 as u8;
        test[4] = 0x80 as u8; test[5] = 0x14 as u8;
        test[6] = 0x30 as u8; test[7] = 0x0C as u8;
        test[8] = 0x80 as u8; test[9] = 0x14 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 5);
        assert_eq!(chip8.registers[1], 6);

        chip8.run();
        assert_eq!(chip8.registers[0], 17);
        assert_eq!(chip8.registers[1], 6);
    }

    #[test]
    fn test_skip_if_not_equal() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 5;
        chip8.registers[1] = 6;

        test[0] = 0x40 as u8; test[1] = 0x06 as u8;
        test[2] = 0x80 as u8; test[3] = 0x14 as u8;
        test[4] = 0x80 as u8; test[5] = 0x14 as u8;
        test[6] = 0x40 as u8; test[7] = 0x0B as u8;
        test[8] = 0x80 as u8; test[9] = 0x14 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 5);
        assert_eq!(chip8.registers[1], 6);

        chip8.run();
        assert_eq!(chip8.registers[0], 17);
        assert_eq!(chip8.registers[1], 6);
    }
}
