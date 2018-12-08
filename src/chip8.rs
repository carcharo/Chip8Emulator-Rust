 use memory::Memory;

pub struct Chip8 {
    memory: Memory,
}

impl Chip8{
    pub fn new() -> Chip8 {
        Chip8 {
            memory: Memory::new()
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        let offset = 0x200;
        for i in 0..data.len() {
            self.memory.write_byte((offset + i) as u16, data[i]);
        }
    }

}