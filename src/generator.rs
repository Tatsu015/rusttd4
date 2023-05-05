use crate::{
    opecode::Opecode,
    tokenizer::{Kind, Token},
};

pub fn generate(tokens: &Vec<Token>) -> Box<[u8]> {
    println!("{:#?}", tokens);
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
    bite_code.clone().into_boxed_slice()
}
