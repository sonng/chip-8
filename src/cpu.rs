mod comparison;
mod subroutine;
mod register_operations;
mod register_i;
mod misc;
mod display;

extern crate rand;

use rand::Rng;

pub struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    program_counter: usize,
    stack: [u16; 16],
    stack_pointer: usize,
    i: u16,
    seed: [u64; 4],
    display: [[bool; 32]; 64],
}

const PROGRAM_START_ADDR: usize = 0x200 as usize;

const MISC: u8 = 0x0 as u8;
const SUBROUTINE: u8 = 0x2 as u8;
const ENDROUTINE: u8 = 0xEE as u8;
const JUMP: u8 = 0x1 as u8;
const SKIP_IF_EQUAL: u8 = 0x3 as u8;
const SKIP_IF_NOT_EQUAL: u8 = 0x4 as u8;
const SKIP_IF_REGISTER_EQUAL: u8 = 0x5 as u8;
const STORE_VALUE_TO_REGISTER: u8 = 0x6 as u8;
const ADD_VALUE_TO_REGISTER: u8 = 0x7 as u8;
const REGISTER_OPERATION: u8 = 0x8 as u8;
const SKIP_IF_REGISTER_NOT_EQUAL: u8 = 0x9 as u8;
const STORE_ADDR_I: u8 = 0xA as u8;
const JUMP_ADDR_PLUS_V0: u8 = 0xB as u8;
const RANDOM_AND: u8 = 0xC as u8;
const DISPLAY: u8 = 0xD as u8;

// Register Actions
const REGISTER_STORE: u8 = 0x0 as u8;
const REGISTER_OR: u8 = 0x1 as u8;
const REGISTER_AND: u8 = 0x2 as u8;
const REGISTER_XOR: u8 = 0x3 as u8;
const REGISTER_ADD: u8 = 0x4 as u8;
const REGISTER_SUB: u8 = 0x5 as u8;
const REGISTER_SHIFT_RIGHT: u8 = 0x6 as u8;
const REGISTER_SUBN: u8 = 0x7 as u8;
const REGISTER_SHIFT_LEFT: u8 = 0xE as u8;


impl CPU {
    pub fn new() -> Self {
        let seeds: [u64; 4] = [
            rand::thread_rng().gen::<u64>(),
            rand::thread_rng().gen::<u64>(),
            rand::thread_rng().gen::<u64>(),
            rand::thread_rng().gen::<u64>(),
        ];

        CPU { 
            registers: [0; 16], 
            memory: [0; 4096], 
            program_counter: 0, 
            stack: [0; 16], 
            stack_pointer: 0, 
            i: 0,
            seed: seeds,
            display: [[false; 32]; 64],
        }
    }

    fn blank_program(&mut self) -> [u8; 3176] {
        [0; 3176]
    }

    fn advance_counter(&mut self) {
        self.program_counter += 2;
    }

    pub fn load(&mut self, program: [u8; 3176]) {
        let mut program_counter = PROGRAM_START_ADDR;
        for e in program.iter() {
            self.memory[program_counter] = *e;
            program_counter += 1
        }

        self.program_counter = PROGRAM_START_ADDR;
    }
    
    fn set_seed(&mut self, seed: [u64; 4]) {
        self.seed = seed;
    }

    pub fn run(&mut self) {
        loop {
            if self.program_counter >= self.memory.len() {
                println!("End of memory, exiting..\n");
                return;
            }

            let op_byte_1 = self.memory[self.program_counter] as u16;
            let op_byte_2 = self.memory[self.program_counter + 1] as u16;
            let op = op_byte_1 << 8 | op_byte_2;

            let op_code = ((op & 0xF000) >> 12) as u8;
            let x = ((op & 0x0F00) >> 8) as usize;
            let y = ((op & 0x00F0) >> 4) as usize;
            let value = (op & 0x000F) as u8;
            let addr = (op & 0x0FFF) as u16;
            let byte = (op & 0x00FF) as u8;

            self.advance_counter();

            match op_code {
                MISC => {
                    match byte {
                        ENDROUTINE => { self.ret(); },
                        _ => {
                            println!("0x0 op code at {:04x}, exiting now..\n", self.program_counter);
                            return;
                        },
                    }
                },
                JUMP => { self.jump(addr); },
                SUBROUTINE => { self.call(addr); },
                SKIP_IF_EQUAL => { self.skip_if_equal(x, byte); },
                SKIP_IF_NOT_EQUAL => { self.skip_if_not_equal(x, byte); },
                SKIP_IF_REGISTER_EQUAL => { self.skip_if_registers_equal(x, y); },
                SKIP_IF_REGISTER_NOT_EQUAL => { self.skip_if_registers_not_equal(x, y); },
                STORE_VALUE_TO_REGISTER => { self.store_register(x, byte); },
                ADD_VALUE_TO_REGISTER => { self.add_register(x, byte); },
                REGISTER_OPERATION => {

                    match value {
                        REGISTER_STORE => { self.copy(x, y); },
                        REGISTER_OR => { self.or(x, y); },
                        REGISTER_AND => { self.and(x, y); },
                        REGISTER_XOR => { self.xor(x, y); },
                        REGISTER_ADD => { self.add(x, y); },
                        REGISTER_SUB => { self.sub(x, y); },
                        REGISTER_SHIFT_RIGHT => { self.shift_right(x, y); },
                        REGISTER_SUBN => { self.subn(x, y); },
                        REGISTER_SHIFT_LEFT => { self.shift_left(x, y); },
                        _ => unimplemented!("No imple for {:04x} - {:04x}", op_code, value),
                    }

                }
                STORE_ADDR_I => { self.store_register_i(addr); },
                JUMP_ADDR_PLUS_V0 => { self.jump_add_v0(addr); },
                RANDOM_AND => { self.random(x, byte); },
                DISPLAY => { self.draw(x, y, value); },
                _ => unimplemented!("No imple for {:04x}", op_code),
            }
        }
    }
}
