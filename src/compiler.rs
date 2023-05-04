use crate::generator::generate;
use crate::tokenizer::tokenize;

use std::fs::File;
use std::io::Read;
use std::io::Write;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile(&self, file_path: &str) {
        let mut in_file = File::open(file_path).unwrap();
        let mut code = String::new();
        in_file.read_to_string(&mut code).unwrap();

        let tokens = tokenize(&code).unwrap();
        let bin = generate(&tokens);

        let mut out_file = File::create("./test").unwrap();
        out_file.write_all(&bin).unwrap();
    }
}