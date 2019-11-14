const PROGRAM_START_ADDR: usize = 0x200 as usize;

pub struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    cur_pos: usize
}

impl CPU {
    pub fn new() -> Self {
        CPU { registers: [0; 16], memory: [0; 4096], cur_pos: 0 }
    }

    pub fn load(&mut self, program: [u8; 3176]) {
        let mut cur_pos = PROGRAM_START_ADDR;
        for e in program.iter() {
            self.memory[cur_pos] = *e;
            cur_pos += 1
        }

        self.cur_pos = PROGRAM_START_ADDR;
    }

    pub fn run(&mut self) {
        loop {
            if self.cur_pos >= self.memory.len() {
                panic!("Current Position overflows");
            }

            let op_byte_1 = self.memory[self.cur_pos] as u16;
            let op_byte_2 = self.memory[self.cur_pos + 1] as u16;
            let op = op_byte_1 << 8 | op_byte_2;

            let op_code = ((op & 0xF000) >> 12) as u8;
            let reg_x = ((op & 0x0F00) >> 8) as u8;
            let reg_y = ((op & 0x00F0) >> 4) as u8;

            self.cur_pos += 2;

            match op_code {
                _ => unimplemented!("No imple for {:04x}", op_code),
            }
        }
    }
}
