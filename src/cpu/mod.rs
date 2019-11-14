mod register_operations;

use std::fmt;
use self::register_operations::*;

const PROGRAM_START_ADDR: usize = 0x200 as usize;

const END_PROGRAM: u8 = 0x0 as u8;
// Register Operations
const REGISTER_OPERATION: u8 = 0x8 as u8;
const REGISTER_STORE: u8 = 0x0 as u8;
const REGISTER_OR: u8 = 0x1 as u8;
const REGISTER_AND: u8 = 0x2 as u8;
const REGISTER_XOR: u8 = 0x3 as u8;

pub struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    cur_pos: usize
}

impl CPU {
    pub fn new() -> Self {
        CPU { registers: [0; 16], memory: [0; 4096], cur_pos: 0 }
    }

    pub fn blank_program(&mut self) -> [u8; 3176] {
        [0; 3176]
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
                println!("End of memory, exiting..\n");
                return;
            }

            let op_byte_1 = self.memory[self.cur_pos] as u16;
            let op_byte_2 = self.memory[self.cur_pos + 1] as u16;
            let op = op_byte_1 << 8 | op_byte_2;

            let op_code = ((op & 0xF000) >> 12) as u8;
            let reg_x = ((op & 0x0F00) >> 8) as u8;
            let reg_y = ((op & 0x00F0) >> 4) as u8;

            self.cur_pos += 2;

            match op_code {
                END_PROGRAM => {
                    println!("0x0 op code at {:04x}, exiting now..\n", self.cur_pos);
                    return;
                },
                REGISTER_OPERATION => {
                    let op_action = (op & 0x000F) as u8;

                    match op_action {
                        REGISTER_STORE => { self.copy(reg_x as usize, reg_y as usize); },
                        REGISTER_OR => { self.or(reg_x as usize, reg_y as usize); },
                        REGISTER_AND => { self.and(reg_x as usize, reg_y as usize); },
                        REGISTER_XOR => { self.xor(reg_x as usize, reg_y as usize); },
                        _ => unimplemented!("No imple for {:04x} - {:04x}", op_code, op_action),
                    }

                }
                _ => unimplemented!("No imple for {:04x}", op_code),
            }
        }
    }
}
