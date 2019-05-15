use crate::chip8_cpu::System;

use std::io::prelude::*;
use std::fs;

#[test]
fn test_loading_rom() {
    let mut chip = System::default();

    chip.load_program(&String::from("test.rom"));

    let original_data = fs::read("test.rom");

    assert!(original_data.is_ok());
    match original_data {
        Ok(data) => {
            let mut i = 0usize;

            for byte in chip.memory[0x200..0x20F].iter() {
                assert!(byte.eq(&data[i]),"BYTES LOADED ARE NOT EQUAL");
                i = i + 1;
            }
        },
        _ => {
            println!("Could not open test.rom");
        }
    }
}

/*0x6XNN where X = Register number and NN is the constant value*/
#[test]
fn test_register_set() {
    let mut chip = System::default();
    chip.memory[0x200] = 0x6A;
    chip.memory[0x201] = 0x02;

    chip.run_op_at(0x200);

    assert_eq!(chip.registers[0xA], 2);
}

#[test]
fn test_set_index_register() {
    let mut chip = System::default();
    chip.memory[0x200] = 0xA2;
    chip.memory[0x201] = 0xEA;

    chip.run_op_at(0x200);

    assert_eq!(chip.index_register, 746);
}

//TODO: Implement screen test