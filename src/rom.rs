use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Rom {
    program: Vec<u8>,
}

impl Rom {
    pub fn new(program_path: &str) -> Result<Rom, io::Error> {
        let mut f = File::open(program_path)?;
        let mut program = Vec::new();
        f.read_to_end(&mut program)?;
        let rom = Rom { program: program };
        return Ok(rom);
    }
    pub fn get_instruction(&self, adress: u8) -> u8 {
        return self.program[adress as usize];
    }
}
