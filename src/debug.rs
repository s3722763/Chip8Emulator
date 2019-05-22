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
    let current_instruction = chip8_system.memory[chip8_system.program_counter as usize];

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
    }).expect("Error displaying debug ui");
}
