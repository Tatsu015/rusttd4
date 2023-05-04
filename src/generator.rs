use crate::{
    opecode::Opecode,
    tokenizer::{Kind, Token},
};

pub fn generate(tokens: &Vec<Token>) -> Box<[u8]> {
    println!("{:#?}", tokens);
    let mut mnimmonics: Vec<u8> = Vec::new();
    for t in tokens {
        match t.kind {
            Kind::Opecode => {
                let opecode = Opecode::str_to_u8(&t.val).unwrap();
                mnimmonics.push(opecode);
            }
            Kind::Operand => {
                let val = u8::from_str_radix(&t.val, 2).unwrap();
                mnimmonics.push(val);
            }
        }
    }
    return mnimmonics.into_boxed_slice();
}
