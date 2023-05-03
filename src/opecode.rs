use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Opecode {
    AddA = 0x00,
    MovAB = 0x01,
    InA = 0x02,
    MovA = 0x03,
    MovBA = 0x04,
    AddB = 0x05,
    InB = 0x06,
    MovB = 0x07,
    OutB = 0x09,
    Out = 0x0b,
    Jnc = 0x0c,
    Jmp = 0x0f,
}

impl Opecode {
    pub fn from_str(opecode: &str) -> Option<Self> {
        match opecode {
            "ADD A" => Some(Opecode::AddA),
            "MOV AB" => Some(Opecode::MovAB),
            "IN A" => Some(Opecode::InA),
            "MOV A" => Some(Opecode::MovA),
            "MOV BA" => Some(Opecode::MovBA),
            "ADD B" => Some(Opecode::AddB),
            "IN B" => Some(Opecode::InB),
            "MOV B" => Some(Opecode::MovB),
            "OUT B" => Some(Opecode::OutB),
            "OUT" => Some(Opecode::Out),
            "JNC" => Some(Opecode::Jnc),
            "JMP" => Some(Opecode::Jmp),
            _ => None,
        }
    }
}
