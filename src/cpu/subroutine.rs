use super::CPU;

impl CPU {
    pub(super) fn call(&mut self, addr: u16) {
        if self.cur_stack >= self.stack.len() {
            panic!("No more space on the stack");
        }

        self.stack[self.cur_stack] = self.program_counter as u16;
        self.cur_stack += 1;
        self.program_counter = addr as usize;
    }

    pub(super) fn ret(&mut self) {
        if self.cur_stack <= 0 {
            panic!("No more subroutine to return to");
        }

        self.cur_stack -= 1;
        self.program_counter = self.stack[self.cur_stack] as usize;
        self.stack[self.cur_stack] = 0;
    }

    pub(super) fn jump(&mut self, addr: u16) {
        self.program_counter = addr as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subroutine_and_return() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 5;
        chip8.registers[1] = 6;

        test[0] = 0x22 as u8; test[1] = 0x0C as u8;
        test[2] = 0x80 as u8; test[3] = 0x14 as u8;
        test[12] = 0x80 as u8; test[13] = 0x14 as u8;
        test[14] = 0x00 as u8; test[15] = 0xEE as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 5);
        assert_eq!(chip8.registers[1], 6);

        chip8.run();
        assert_eq!(chip8.registers[0], 17);
        assert_eq!(chip8.registers[1], 6);
    }

    #[test]
    #[should_panic]
    fn test_return_with_no_stack() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        test[0] = 0x00 as u8; test[1] = 0xEE as u8;

        chip8.load(test);
        chip8.run();
    }

    #[test]
    #[should_panic]
    fn test_subroutine_overflow() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        test[0] = 0x22 as u8; test[1] = 0x00 as u8;

        chip8.load(test);
        chip8.run();
    }

    #[test]
    fn test_jump() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.registers[0] = 5;
        chip8.registers[1] = 6;

        test[0] = 0x12 as u8; test[1] = 0x0C as u8;
        test[2] = 0x80 as u8; test[3] = 0x14 as u8;
        test[12] = 0x80 as u8; test[13] = 0x14 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 5);
        assert_eq!(chip8.registers[1], 6);

        chip8.run();
        assert_eq!(chip8.registers[0], 11);
        assert_eq!(chip8.registers[1], 6);
    }
}


