use crate::compile::generator::generate;
use crate::compile::tokenizer::tokenize;

use std::fs::File;
use std::io::Read;
use std::io::Write;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile(&self, src_file_path: &str, dst_file_path: &str) {
        let mut in_file = File::open(src_file_path).unwrap();
        let mut code = String::new();
        in_file.read_to_string(&mut code).unwrap();

        let tokens = tokenize(&code).unwrap();
        let bin = generate(&tokens);

        let mut out_file = File::create(dst_file_path).unwrap();
        out_file.write_all(&bin).unwrap();
    }
}
