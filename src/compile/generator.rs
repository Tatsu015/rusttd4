use crate::compile::tokenizer::{Kind, Token};
use crate::opecode::Opecode;

pub fn generate(tokens: &Vec<Token>) -> Box<[u8]> {
    let mut bite_code: Vec<u8> = Vec::new();
    for t in tokens {
        match t.kind {
            Kind::Opecode => {
                let opecode = Opecode::str_to_u8(&t.val).unwrap();
                bite_code.push(opecode);
            }
            Kind::Operand => {
                let val = u8::from_str_radix(&t.val, 2).unwrap();
                bite_code.push(val);
            }
        }
    }

    let squashed = squash(&bite_code);
    squashed
}

fn squash(bite_code: &Vec<u8>) -> Box<[u8]> {
    let mut squashed: Vec<u8> = Vec::new();
    for pair in bite_code.chunks(2) {
        let t: u8 = (pair[0] << 4) | pair[1];
        squashed.push(t);
    }
    squashed.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use crate::generator::generate;
    use crate::tokenizer::{Kind, Token};

    #[test]
    fn success_to_generate_bite_code() {
        let tokens = vec![
            Token {
                kind: Kind::Opecode,
                val: "OUT".to_string(),
            },
            Token {
                kind: Kind::Operand,
                val: "1010".to_string(),
            },
        ];
        let bite = generate(&tokens);
        assert_eq!(&bite[..], &vec![0b10111010]);
    }
}
