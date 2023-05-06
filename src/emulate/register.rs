const REGISTER_CAPACITY: u8 = 0x0f;
pub struct Register {
    val: u8,
}

impl Register {
    pub fn new() -> Register {
        return Register { val: 0 };
    }

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

    pub fn is_overflow(&self, val: u8) -> bool {
        if val > REGISTER_CAPACITY {
            return true;
        } else {
            return false;
        }
    }
}
