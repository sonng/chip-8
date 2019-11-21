use super::CPU;
use bitvec::prelude::*;

impl CPU {
    pub(super) fn draw(&mut self, x: usize, y: usize, byte: u8) {
        let mut lower_bound = self.i as usize;
        let upper_bound = lower_bound + (byte as usize);

        let mut cur_y = self.registers[y] as usize;
        while lower_bound < upper_bound {
            let value: u8 = self.memory[lower_bound];
            let bv = BitVec::<BigEndian, u8>::from_element(value);

            let mut cur_x = self.registers[x] as usize;
            
            for bit in bv {
                let previous = self.display[cur_x][cur_y];
                let new = previous != bit;
                println!("({}, {}) - {} to {}", cur_x, cur_y, previous, new);

                self.display[cur_x][cur_y] = new; 

                if new == false { 
                    self.registers[0xF] = 0x1 as u8;
                }

                cur_x += 1;
            }

            cur_y += 1;
            lower_bound += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_sprite() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        test[00] = 0x62 as u8; test[01] = 0x0A as u8;
        test[02] = 0x63 as u8; test[03] = 0x0C as u8;
        test[04] = 0xA2 as u8; test[05] = 0x20 as u8;
        test[06] = 0xD2 as u8; test[07] = 0x36 as u8;

        test[32] = 0xBA as u8; test[33] = 0x7C as u8;
        test[34] = 0xD6 as u8; test[35] = 0xFE as u8;
        test[36] = 0x54 as u8; test[37] = 0xAA as u8;

        chip8.load(test);

        chip8.run();
        assert_eq!(chip8.display[10][12], true);
        assert_eq!(chip8.display[11][12], false);
        assert_eq!(chip8.display[12][12], true);
        assert_eq!(chip8.display[13][12], true);
        assert_eq!(chip8.display[14][12], true);
        assert_eq!(chip8.display[15][12], false);
        assert_eq!(chip8.display[16][12], true);
        assert_eq!(chip8.display[17][12], false);
        assert_eq!(chip8.display[10][13], false);
        assert_eq!(chip8.display[11][13], true);
        assert_eq!(chip8.display[12][13], true);
        assert_eq!(chip8.display[13][13], true);
        assert_eq!(chip8.display[14][13], true);
        assert_eq!(chip8.display[15][13], true);
        assert_eq!(chip8.display[16][13], false);
        assert_eq!(chip8.display[17][13], false);
        assert_eq!(chip8.display[10][14], true);
        assert_eq!(chip8.display[11][14], true);
        assert_eq!(chip8.display[12][14], false);
        assert_eq!(chip8.display[13][14], true);
        assert_eq!(chip8.display[14][14], false);
        assert_eq!(chip8.display[15][14], true);
        assert_eq!(chip8.display[16][14], true);
        assert_eq!(chip8.display[17][14], false);
        assert_eq!(chip8.display[10][15], true);
        assert_eq!(chip8.display[11][15], true);
        assert_eq!(chip8.display[12][15], true);
        assert_eq!(chip8.display[13][15], true);
        assert_eq!(chip8.display[14][15], true);
        assert_eq!(chip8.display[15][15], true);
        assert_eq!(chip8.display[16][15], true);
        assert_eq!(chip8.display[17][15], false);
        assert_eq!(chip8.display[10][16], false);
        assert_eq!(chip8.display[11][16], true);
        assert_eq!(chip8.display[12][16], false);
        assert_eq!(chip8.display[13][16], true);
        assert_eq!(chip8.display[14][16], false);
        assert_eq!(chip8.display[15][16], true);
        assert_eq!(chip8.display[16][16], false);
        assert_eq!(chip8.display[17][16], false);
        assert_eq!(chip8.display[10][17], true);
        assert_eq!(chip8.display[11][17], false);
        assert_eq!(chip8.display[12][17], true);
        assert_eq!(chip8.display[13][17], false);
        assert_eq!(chip8.display[14][17], true);
        assert_eq!(chip8.display[15][17], false);
        assert_eq!(chip8.display[16][17], true);
        assert_eq!(chip8.display[17][17], false);
    }
}
