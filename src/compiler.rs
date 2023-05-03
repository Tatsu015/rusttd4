use std::fs::File;
use std::io::Read;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile(&self, file_path: &str) {
        let mut file = File::open(file_path).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        println!("\r{}", code);
    }
}
