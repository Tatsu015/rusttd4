pub enum Opecode {
    AddA,
    MovAB,
    InA,
    MovA,
    MovBa,
    AddB,
    InB,
    MovB,
    OutB,
    Out,
    Jnc,
    Jmp,
}

impl Opecode {
    pub fn from_u8(opecode: u8) -> Option<Self> {
        match opecode {
            0x00 => Some(Opecode::AddA),
            0x01 => Some(Opecode::MovAB),
            0x02 => Some(Opecode::InA),
            0x03 => Some(Opecode::MovA),
            0x04 => Some(Opecode::MovBa),
            0x05 => Some(Opecode::AddB),
            0x06 => Some(Opecode::InB),
            0x07 => Some(Opecode::MovB),
            0x09 => Some(Opecode::OutB),
            0x0b => Some(Opecode::Out),
            0x0c => Some(Opecode::Jnc),
            0x0f => Some(Opecode::Jmp),
            _ => None,
        }
    }
}
