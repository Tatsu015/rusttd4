use crate::tokenizer::Token;

pub fn generate(tokens: &Vec<Token>) -> &[u8] {
    println!("{:#?}", tokens);
    return &[1u8, 2u8];
}
