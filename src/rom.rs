pub struct Rom {
    pub program: Vec<u8>,
}

impl Rom {
    pub fn get_instruction(&self, adress: u8) -> u8 {
        return self.program[adress as usize];
    }
}
