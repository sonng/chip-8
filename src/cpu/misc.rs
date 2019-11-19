use super::CPU;

extern crate rand;
use rand::{ Rng, SeedableRng };
use rand::rngs::StdRng;

use std::mem::transmute;

impl CPU {
    pub(super) fn random(&mut self, x: usize, value: u8) {
        let seeds: [u8; 32] = unsafe { transmute::<[u64; 4], [u8; 32]>(self.seed) };
        let mut rng: StdRng = SeedableRng::from_seed(seeds);
        let random: u8 = rng.gen::<u8>();

        println!("Random: {}, Value: {}", random, value);

        self.registers[x] = random & value;

        self.seed = [
            rng.gen::<u64>(),
            rng.gen::<u64>(),
            rng.gen::<u64>(),
            rng.gen::<u64>(),
        ];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let mut chip8 = CPU::new();
        let mut test = chip8.blank_program();

        chip8.set_seed([1, 2, 3, 4]);
        chip8.registers[0] = 5;

        test[0] = 0xC0 as u8; test[1] = 0x07 as u8;

        chip8.load(test);
        assert_eq!(chip8.registers[0], 5);

        chip8.run();
        assert_eq!(chip8.registers[0], 7);
    }
}
