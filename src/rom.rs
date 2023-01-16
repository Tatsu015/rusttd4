use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Rom {
    program: Vec<u8>,
}

impl Rom {
    pub fn new(program_path: &str) -> Rom {
        let f = match File::open(program_path) {
            Ok(file) => file,
            Err(error) => {
                panic!("Can not open program file : {:?}", program_path);
            }
        };

        let mut program = Vec::new();
        let res = match f.read_to_end(&mut program) {
            Ok(s) => s,
            Err(err) => {
                panic!("Can not read program file : {:?}", program_path);
            }
        };
        let rom = Rom { program: program };
        return rom;
    }

    pub fn get_instruction(&self, adress: u8) -> u8 {
        return self.program[adress as usize];
    }
}
