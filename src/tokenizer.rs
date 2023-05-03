#[derive(Debug, PartialEq)]
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

pub fn tokenize(code: &str) -> Result<Vec<Token>, String> {
    let mnemonics: Vec<String> = code.lines().map(|l| l.trim().to_owned()).collect();
    let tokens: Result<Vec<_>, _> = mnemonics.iter().map(|m| to_token(&m)).collect();
    tokens.and_then(|v| Ok(v.into_iter().flatten().collect()))
}

fn to_token(mnemonic: &str) -> Result<Vec<Token>, String> {
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

    if elms.len() < 2 {
        return Err(format!("Syntax error{}", mnemonic));
    }

    // splitted first element become opecode, and second element become operand
    let c = elms.get(0).unwrap();
    let opecode = Token::new(Kind::Opecode, c.to_string());

    let r = elms.get(1).unwrap();
    let operand = Token::new(Kind::Operand, r.to_string());

    Ok(vec![opecode, operand])
}

// #[cfg(test)]
// mod tests {
//     use crate::tokenizer::{tokenize, Kind};

//     #[test]
//     fn tokenize_2_mnimonic() {
//         let tokens = tokenize("OUT 0011\nADD A,0001\n");
//         assert_eq!(tokens.len(), 4);
//         assert_eq!(tokens[0], Kind::Opecode);
//         // assert_eq!(tokens[0].val, "OUT");
//         // assert_eq!(tokens[1].kind, Kind::Operand);
//         // assert_eq!(tokens[1].val, "0011");
//         // assert_eq!(tokens[2].kind, Kind::Opecode);
//         // assert_eq!(tokens[2].val, "ADD A");
//         // assert_eq!(tokens[3].kind, Kind::Operand);
//         // assert_eq!(tokens[3].val, "0001");
//     }

//     #[test]
//     fn opecode_only() {
//         let tokens = tokenize("OUT");
//         let tokens = tokenize("INVALID");
//         assert!(tokens.is_err());
//     }
// }
