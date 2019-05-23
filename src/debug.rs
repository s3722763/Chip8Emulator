extern crate tui;
extern crate termion;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Style};
use tui::widgets::{Block, List, Borders, Text, Widget};
use tui::Terminal;

use crate::chip8_cpu::System;
use self::tui::layout::Corner;
use std::io;

/**Why does this return have to be so looooooooooooooooong**/
pub fn setup_debug_ui() -> Terminal<tui::backend::TermionBackend<termion::screen::AlternateScreen<termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>>>> {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    terminal
}

pub fn update_and_display_debug_ui(terminal :&mut Terminal<tui::backend::TermionBackend<termion::screen::AlternateScreen<termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>>>>,
                               chip8_system: &System) {
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

        system_status_vec.push(format!("Program counter: {:X}", chip8_system.program_counter));
        system_status_vec.push(format!("Current instruction: {}", current_instruction));

        let system_status = system_status_vec.iter().map(|value| {
            Text::raw(value)
        });

        List::new(system_status)
            .block(Block::default().borders(Borders::ALL).title("System Status"))
            .start_corner(Corner::TopLeft)
            .render(&mut f, chunks[2]);
    }).expect("Error displaying debug ui");
}

fn get_opcode_description(opcode_first: u8, opcode_second: u8, system: &System) -> String {
    let result = String::new();

    match first {
        0x00 => { self.process_0x_00(value, second); },
        0x10 => { format!("Jump to {}{}", (first & 0x0F), second); },
        0x20 => { format!("Call subroutine {}{}", (first & 0x0F), second); },
        0x30 => { format!("Skip next instruction if value at register {}, Value {} is equal to constant {}"
                              (first & 0x0F), system.registers[first & 0x0F as usize], second)},
        0x40 => { unimplemented!("Skip if not equal (constant)"); },
        0x50 => { unimplemented!("Skip if equal (to register)"); },
        0x60 => { format("Set {}{}") },
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

    result
}