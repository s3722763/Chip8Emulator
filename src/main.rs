extern crate sdl2;

#[cfg(debug_assertions)]
mod debug;

mod chip8_cpu;

use chip8_cpu::System;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[cfg(test)]
pub mod tests;

fn setup_window() -> (Canvas<Window>, sdl2::EventPump){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip 8 Emulator", 64, 32)
        .position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    (canvas,event_pump)
}

fn main() {
    let mut chip8_system: System = System::default();

    chip8_system.load_program(&String::from("pong.rom"));
    //chip8_system.load_program(&String::from("test.rom"));
    chip8_system.setup_fontset();

    let (mut canvas, mut event_pipe) = setup_window();
    let mut terminal = None;

    if cfg!(debug_assertions) {
        terminal =  Some(debug::setup_debug_ui());
    }

    'running: loop {
        if cfg!(debug_assertions) {
            debug::update_and_display_debug_ui(&mut terminal.as_mut().unwrap(), &chip8_system);
        }

       chip8_system.run_op_at(chip8_system.program_counter);
        /**Handle SDL2 events and drawing**/
        canvas.clear();
        for event in event_pipe.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        for x in 0..64 {
            for y in 0..32 {
                if chip8_system.screen[x][y] == 1 {
                    let point = sdl2::rect::Point::new(x as i32, y as i32);

                    canvas.draw_point(point);
                }
            }
        }

        canvas.present();
    }

    /*for byte in chip8_system.memory[0x200..0x210].iter() {
        println!("{}", byte);
    }*/
}
