mod chip8_cpu;

use chip8_cpu::System;

#[cfg(test)]
pub mod tests;

fn main() {
    let mut chip8_system: System = System::default();
    let mut running = true;

    chip8_system.load_program(&String::from("pong.rom"));

    while running {
        chip8_system.run_op_at(chip8_system.program_counter);
    }

    /*for byte in chip8_system.memory[0x200..0x210].iter() {
        println!("{}", byte);
    }*/
}
