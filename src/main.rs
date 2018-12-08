
use std::io::prelude::*;
use std::fs::File;
use chip8::Chip8;

mod memory;
mod chip8;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("could not read file");

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

}
