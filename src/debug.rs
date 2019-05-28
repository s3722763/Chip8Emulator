extern crate tui;
extern crate rustbox;
extern crate termion;

use tui::backend::RustboxBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Style};
use tui::widgets::{Block, List, Borders, Text, Widget};
use tui::Terminal;

use termion::input::Events;
use termion::input::TermRead;
use termion::event::Key;

use crate::chip8_cpu::System;
use self::tui::layout::Corner;
use std::io;
use std::any::Any;
use std::io::stdin;

/**Why does this return have to be so looooooooooooooooong**/
pub fn setup_debug_ui() -> Terminal<tui::backend::RustboxBackend> {
    let backend = RustboxBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    terminal
}

pub fn update_and_display_debug_ui(terminal :&mut Terminal<tui::backend::RustboxBackend>,
                               chip8_system: &System) -> bool {
    let mut quit = false;
    //Poll events
    let key_input = stdin();
    for c in key_input.keys() {
        match c.unwrap() {
            Key::Char('q') => { quit = true; }
            _ => {}
        }
    }


    terminal.draw(|mut f| {
        let chunks = Layout::default().direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(20), Constraint::Percentage(60)].as_ref())
            .split(f.size());

        let mut i = -1;
        let registers = chip8_system.registers.iter().map(|register| {
            i += 1;
            Text::styled(format!("{:X} - {}", i, register), Style::default())
        });

        List::new(registers)
            .block(Block::default().borders(Borders::ALL).title("Registers"))
            .start_corner(Corner::TopLeft)
            .render(&mut f, chunks[0]);

        let stack = chip8_system.stack.iter().rev().map(|value| {
            Text::raw(format!("{}", value))
        });

        List::new(stack)
            .block(Block::default().borders(Borders::ALL).title("Stack"))
            .start_corner(Corner::TopLeft)
            .render(&mut f, chunks[1]);

        let mut system_status_vec: Vec<String> = Vec::new();
        //TODO: Add more system status stuff
        let current_instruction_1 = chip8_system.memory[chip8_system.program_counter as usize];
        let current_instruction_2 = chip8_system.memory[(chip8_system.program_counter + 1) as usize];

        let current_instruction = format!("{:X}{:X}", current_instruction_1, current_instruction_2);
        let instruction_description = get_opcode_description(current_instruction_1, current_instruction_2, chip8_system);

        system_status_vec.push(format!("Program counter: {:X}", chip8_system.program_counter));
        system_status_vec.push(format!("Current instruction: {}", current_instruction));
        system_status_vec.push(format!("Current instruction description: {}", instruction_description));

        let system_status = system_status_vec.iter().map(|value| {
            Text::raw(value)
        });

        List::new(system_status)
            .block(Block::default().borders(Borders::ALL).title("System Status"))
            .start_corner(Corner::TopLeft)
            .render(&mut f, chunks[2]);
    }).expect("Error displaying debug ui");

    quit
}

fn get_opcode_description(opcode_first: u8, opcode_second: u8, system: &System) -> String {
    let mut result = String::new();
    let first = opcode_first & 0xF0;

    match first {
        0x00 => {
            match opcode_second {
                0xE0 => { result = format!("Clear screen"); },
                0xEE => { result = format!("Return from subroutine, returning to address {}",
                        system.stack[(system.stack_pointer as usize)]);},
                _ => { result = format!("Invalid opcode"); }
            }
        },
        0x10 => { result = format!("Jump to {}{}", (opcode_first & 0x0F), opcode_second); },
        0x20 => { result = format!("Call subroutine {}{}", (first & 0x0F), opcode_second); },
        0x30 => { result = format!("Skip next instruction if value at register {}, Value {} is equal to constant {}",
                              (opcode_first & 0x0F), system.registers[(first & 0x0F) as usize], opcode_second); },
        0x40 => { unimplemented!("Skip if not equal (constant)"); },
        0x50 => { unimplemented!("Skip if equal (to register)"); },
        0x60 => { result = format!("Set value in register {} to value {}", (opcode_first & 0x0F), opcode_second); },
        0x70 => { result = format!("Add value in register {}, value {}", (opcode_first & 0x0F), opcode_second); },
        0x80 => { unimplemented!("Binary ops and maths"); },
        0x90 => { unimplemented!("Skip if registers not equal"); },
        0xA0 => { result = format!("Set index register to {:X}{:X}", (opcode_first & 0x0F), opcode_second); },
        0xB0 => { unimplemented!("Jump to address plus value in V0"); },
        0xC0 => { unimplemented!("Set register to random value anded with NN"); },
        0xD0 => { result = format!("Draw starting from X: {} - Y: {}, drawing {} pixels high", (opcode_first & 0x0F),
                          (opcode_second & 0xF0), (opcode_second & 0x0F)); },
        0xE0 => { unimplemented!("Key operations"); },
        0xF0 => {
            match opcode_second {
                0x07 => { result = format!("Set register {} equal to delay timer value, {}", (opcode_first & 0x0F), system.delay_timer); },
                0x0A => { result = format!("Halt program until a key is pressed and put key value into register {}", (opcode_first & 0x0F)); },
                0x15 => { result = format!("Set delay timer to value in register {}", (opcode_first & 0x0F)); },
                0x18 => { result = format!("Set sound timer to value in register {}", (opcode_first & 0x0F)); },
                0x1E => { result = format!("Add value in register {} to index register value", (opcode_first & 0x0F)); },
                0x29 => { result = format!("Set index register to character location which represents the value {}", (opcode_first & 0x0F)); },
                0x33 => { result = format!("Set value at index register, +1 and +2, to BCD representation of number at register {}", (opcode_first & 0x0F)) },
                0x55 => { result = format!("Store register 0 to register {} into memory starting from index register", (opcode_first & 0x0F)); },
                0x65 => { result = format!("Fill register 0 to register {} into memory starting from index register", (opcode_first & 0x0F)); },
                _ => {
                    result = format!("Invalid opcode");
                }
            }
        },
        _ => {
            result = format!("Invalid opcode");
        }
    }

    result
}