use crate::tokenizer::*;

enum Kind {
    Opecode,
    Operand,
}

struct Token {
    kind: Kind,
    val: String,
}

fn tokenize(code: String) -> Vec<Token> {}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_one_token() {
        let tokens = tokenize("OUT 0011");
        assert_eq!(tokens.len, 2);
    }
}
