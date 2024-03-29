use std::fs::File;
use std::io::prelude::*;

pub struct Rom {
    program: Vec<u8>,
}

impl Rom {
    pub fn new(program_path: &str) -> Rom {
        let mut f = match File::open(program_path) {
            Ok(file) => file,
            Err(error) => {
                panic!("Can not open program file : {:?}", error);
            }
        };

        let mut program = Vec::new();
        let _ = match f.read_to_end(&mut program) {
            Ok(s) => s,
            Err(err) => {
                panic!("Can not read program file : {:?}", err);
            }
        };
        let rom = Rom { program: program };
        return rom;
    }

    pub fn get_instruction(&self, adress: u8) -> u8 {
        return self.program[adress as usize];
    }
}
