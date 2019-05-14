use std::io::prelude::*;
use std::path::Path;
use std::fs;

pub struct System {
    pub memory: [u8;4096],
    pub registers: [u8;16],
    pub index_register: u16,
    pub program_counter: u16,
    screen: [u8;64*32],
    delay_timer: u8,
    sound_timer: u8,
    key: [u8;16],
    //For Emulation
    stack: [u8;16],
    stack_pointer: u8,
}

impl Default for System {
    fn default() -> System {
        System {
            memory: [0;4096],
            registers: [0;16],
            index_register: 0,
            //Stating point of program
            program_counter: 0x200,
            screen: [0;64*32],
            delay_timer: 0,
            sound_timer: 0,
            key: [0;16],
            stack: [0;16],
            stack_pointer: 0
        }
    }
}

impl System {
    fn get_next_op(&self) {

    }

    pub fn run_op_at(&mut self, address: u16) {
        //TODO: Do check if there is an op code
        let value = self.memory[address as usize];
        let first =  value & 0xF0;
        let mut address_changed = false;
        let second = self.memory[(address + 1) as usize];

        match first {
            0x00 => { unimplemented!("Call, display_clear or return"); },
            0x10 => { unimplemented!("Jump to"); },
            0x20 => { unimplemented!("Call"); },
            0x30 => { unimplemented!("Skip if equal (constant)"); },
            0x40 => { unimplemented!("Skip if not equal (constant)"); },
            0x50 => { unimplemented!("Skip if equal (to register)"); },
            0x60 => { self.set_register_to(value, second); },
            0x70 => { unimplemented!("Add constant to register value"); },
            0x80 => { unimplemented!("Binary ops and maths"); },
            0x90 => { unimplemented!("Skip if registers not equal"); },
            0xA0 => { self.set_index_register(value, second); },
            0xB0 => { unimplemented!("Jump to address plus value in V0"); },
            0xC0 => { unimplemented!("Set register to random value anded with NN"); },
            0xD0 => { unimplemented!("Draw"); },
            0xE0 => { unimplemented!("Key operations"); },
            0xF0 => { unimplemented!("Other stuff like timer sound memory stuff and conversion stuff"); },
            _ => {
                println!("Invalid opcode");
            }
        }

        if !address_changed {
            self.program_counter = self.program_counter + 2;
        }
    }

    fn set_index_register(&mut self, first_part: u8, second_part: u8) {
        let top_value: u16 = ((first_part & 0x0F) as u16) * 256;
        let total_value = top_value + (second_part as u16);

        self.index_register = total_value;
    }

    fn set_register_to(&mut self, first_part: u8, second_part: u8) {
        let register = (first_part & 0x0F) as usize;

        self.registers[register] = second_part;
    }

    pub fn load_program(&mut self, program_path: &String) {
        match fs::read(program_path) {
            Ok(program) => {
                let mut current_address: u16 = 0x200;

                for byte in program {
                    self.memory[current_address as usize] = byte;
                    current_address = current_address + 1;
                }
            },
            Err(e) => {
                println!("Could not load file");
            }
        }
    }
}