pub enum Kind {
    Opecode,
    Operand,
}

pub struct Token {
    kind: Kind,
    val: String,
}

impl Token {
    pub fn new(kind: Kind, val: String) -> Token {
        Token {
            kind: kind,
            val: val,
        }
    }
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mnemonics: Vec<String> = code.lines().map(str::trim).map(str::to_owned).collect();
    let tokens: _ = mnemonics.iter().map(|m| to_token(&m));

    let flat_tokens: Vec<Token> = tokens.into_iter().flatten().collect();
    flat_tokens
}

fn to_token(mnemonic: &String) -> Vec<Token> {
    // mnemonic have 2 type
    // * OUT 0111
    // this case, split by ' '
    // * ADD A,0001
    // this case, split by ','
    let elms: Vec<&str> = if mnemonic.contains(",") {
        mnemonic.split(",").collect()
    } else {
        mnemonic.split(" ").collect()
    };

    // splitted first element become opecode, and second element become operand
    let c = elms.get(0).expect("syntax error");
    let opecode = Token::new(Kind::Opecode, c.to_string());

    let r = elms.get(1).expect("syntax error");
    let operand = Token::new(Kind::Operand, r.to_string());

    vec![opecode, operand]
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_one_token() {
        let tokens = tokenize("OUT 0011");
        assert_eq!(tokens.len(), 1);
    }
}
