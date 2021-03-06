use std::io::prelude::*;
use std::path::Path;
use std::fs;

pub struct System {
    pub memory: [u8;4096],
    pub registers: [u8;16],
    pub index_register: u16,
    pub program_counter: u16,
    pub screen: [[u8;32];64],
    pub delay_timer: u8,
    pub sound_timer: u8,
    key: [u8;16],
    //For Emulation
    pub stack: [u16;16],
    pub stack_pointer: u8,
}

impl Default for System {
    fn default() -> System {
        System {
            memory: [0;4096],
            registers: [0;16],
            index_register: 0,
            //Stating point of program
            program_counter: 0x200,
            screen: [[0;32];64],
            delay_timer: 0,
            sound_timer: 0,
            key: [0;16],
            stack: [0;16],
            stack_pointer: 0
        }
    }
}

impl System {
    pub fn setup_fontset(&mut self) {
        let chip8_fontset = [ 
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for index in 0..chip8_fontset.len() {
            self.memory[index] = chip8_fontset[index];
        }
    }

    pub fn run_op_at(&mut self, address: u16) {
        //TODO: Do check if there is an op code
        let value = self.memory[address as usize];
        let first =  value & 0xF0;
        let mut address_changed = false;
        let second = self.memory[(address + 1) as usize];
        //println!("New address : {}", address);
        //println!("New op-code: {:x}{:x}\n", value, second);
        match first {
            0x00 => { self.process_0x_00(value, second); },
            0x10 => { self.jump(value, second); },
            0x20 => { self.call(value, second, address); },
            0x30 => { self.skip_if_equal(value, second); },
            0x40 => { unimplemented!("Skip if not equal (constant)"); },
            0x50 => { unimplemented!("Skip if equal (to register)"); },
            0x60 => { self.set_register_to(value, second); },
            0x70 => { self.add_value_to_register(value, second); },
            0x80 => { unimplemented!("Binary ops and maths"); },
            0x90 => { unimplemented!("Skip if registers not equal"); },
            0xA0 => { self.set_index_register(value, second); },
            0xB0 => { unimplemented!("Jump to address plus value in V0"); },
            0xC0 => { unimplemented!("Set register to random value anded with NN"); },
            0xD0 => { self.draw(value, second); },
            0xE0 => { unimplemented!("Key operations"); },
            0xF0 => { self.process_0x_F0(value, second); },
            _ => {
                //println!("Invalid opcode");
            }
        }

        if !address_changed {
            self.program_counter = self.program_counter + 2;
        }
    }

    fn skip_if_equal(&mut self, first: u8, value: u8) {
        let register = first & 0x0F;
        let register_value = self.registers[register as usize];

        if value == register_value {
            //Skip the next instruction as this adds to and when it returns 2 is also added
            self.program_counter += 2;
        }
    }

    fn add_value_to_register(&mut self, first: u8, value: u8) {
        let register = (first & 0x0F) as usize;
        self.registers[register] += value;
    }

    fn process_0x_00(&mut self, first_part: u8, second_part: u8) -> bool{
        let code_redirect = false;

        match second_part {
            0xEE => {
                self.stack_pointer -= 1;
                let new_address = self.stack[self.stack_pointer as usize];

                self.program_counter = new_address;
                //println!("Returning to: {}", new_address);
            },
            _ => {
                unimplemented!("0x00 opcode not implemented");
            }
        }

        code_redirect
    }

    fn process_0x_F0(&mut self, first_part: u8, second_part: u8) {
        match second_part {
            0x07 => {
                //Set VX to equal to delay timer
                let register  = first_part & 0x0F;
                self.registers[register as usize] = self.delay_timer;
            },
            0x15 => {
                //Set delay timer to the value in VX
                let register = first_part & 0x0F;
                let amount = self.registers[register as usize];
                self.delay_timer = amount;
            },
            0x29 => {
                //Set index register to value sprite address what represents this value
                let register = first_part & 0x0F;
                let value = self.registers[register as usize];
                //Is this ok?
                //The index is always 5 times its value that it wants to represent
                self.index_register = (value as u16) * 5;
            },
            0x33 => {
                //Calculate the BCD for this value and store at where index register points to
                let register = (first_part & 0xF0) >> 4;
                let value = self.registers[register as usize];
                let (hundreds, tens, ones) = encode_to_bcd(value);

                self.memory[self.index_register as usize] = hundreds;
                self.memory[(self.index_register + 1) as usize] = tens;
                self.memory[(self.index_register + 2) as usize] = ones;
            },
            0x65 => {
                let last_register = (first_part & 0xF0) >> 4;
                self.reg_load(last_register);
            }
            _ => { unimplemented!("Other 0xF0 opcodes unimplemented"); }
        }
    }

    fn reg_load(&mut self, last_register: u8) {
        //Inclusive of the last value
        for register in 0..(last_register + 1) {
            let value = self.memory[self.index_register as usize];
            self.registers[register as usize] = value;
        }
    }

    fn jump(&mut self, first_part: u8, second_part: u8) {
        let top_value: u16 = ((first_part & 0x0F) as u16) * 256;
        let total_value = top_value + (second_part as u16);

        self.program_counter = total_value;
    }

    fn call(&mut self, first_part: u8, second_part: u8, original_address: u16) {
        //println!("Adding {} to stack", original_address);
        self.stack[self.stack_pointer as usize] = original_address;
        self.stack_pointer = self.stack_pointer + 1;

        self.jump(first_part, second_part);
    }

    fn draw(&mut self, first_part: u8, second_part: u8) {
        let x_register = first_part & 0x0F;
        let y_register = (second_part & 0xF0) >> 4;
        //println!("Draw");
        //println!("Y Register: {}", y_register);
        let height = second_part & 0x0F;
        let initial_height = self.registers[y_register as usize];
        let initial_width = self.registers[x_register as usize];
        //println!("Height: {}", height);
        //Reset 0xF register
        self.registers[0xF] = 0;

        //Get the sprite at this address

        for y in 0..height {
            let sprite_line = self.memory[(self.index_register + y as u16) as usize];
            //println!("{:x}", sprite_line);
            for x in 0..8 {
                let pixel = sprite_line & (0x80 >> x);
                //println!("{:b}", pixel);
                if pixel != 0{
                    //Pixel is now a colour
                    //TODO:Implement collision detection
                }
                //println!("X: {}\tY: {}", x + initial_width as u16, y + initial_height);
                let new_x = x + initial_width as u16;
                let new_y = y + initial_height;

                if new_x < 64 && new_y < 32 {
                    self.screen[new_x as usize][new_y as usize] = pixel;
                }
            }
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
                //println!("Could not load file");
            }
        }
    }
}

fn encode_to_bcd(value: u8) -> (u8, u8, u8){
    let hundreds = value / 100;
    let rest = value % 100;
    let tens = rest / 10;
    let ones = rest % 10;

    (hundreds, tens, ones)
}