pub struct Register {
    val: u8,
}

impl Register {
    pub fn set(&mut self, val: u8) {
        if val > REGISTER_CAPACITY {
            self.val = 0;
        } else {
            self.val = val;
        }
    }

    pub fn get(&self) -> u8 {
        return self.val;
    }
}
