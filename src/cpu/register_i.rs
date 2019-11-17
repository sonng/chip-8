use super::CPU;

impl CPU {
    pub(super) fn store_register_i(&mut self, addr: u16) {
        self.i = addr;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_addr_to_i() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        test[0] = 0xA2 as u8; test[1] = 0x50 as u8;

        chip8.load(test);
        assert_eq!(chip8.i, 0);

        chip8.run();
        assert_eq!(chip8.i, 0x250 as u16);
    }
}
