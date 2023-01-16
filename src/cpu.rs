use crate::register::Register;
use crate::rom::Rom;

pub struct Cpu {
    pc: Register,
    carry: Register,
    a: Register,
    b: Register,
    rom: Rom,
    input: Register,
    output: Register,
}

impl Cpu {
    pub fn new(rom: Rom, input: Register, output: Register) -> Cpu {
        let cpu = Cpu {
            pc: Register::new(),
            carry: Register::new(),
            a: Register::new(),
            b: Register::new(),
            rom: rom,
            input: input,
            output: output,
        };
        return cpu;
    }

    pub fn show(&self) {
        let s = format!("{:0>4b}", self.output.get());
        let s = s.replace("0", "□").replace("1", "■");
        println!("{}", s);
    }

    pub fn fetch(&self) -> u8 {
        log::debug!("Fetch program");
        let ins = self.rom.get_instruction(self.pc.get());
        return ins;
    }

    pub fn pc_up(&mut self) {
        let pc = self.pc.get();
        self.pc.set(pc + 1);
        log::debug!("PC count up")
    }

    pub fn decode(&self, instruction: u8) -> (u8, u8) {
        log::debug!("Decode instruction");
        let imm = instruction & 0x0f;
        let ope = (instruction & 0xf0) >> 4;
        return (ope, imm);
    }

    pub fn execute(&mut self, opecode: u8, immidiate: u8) {
        log::debug!("Execute instruction");
        match opecode {
            0x00 => self.add_a(immidiate),
            0x01 => self.mov_ab(),
            0x02 => self.in_a(immidiate),
            0x03 => self.mov_a(immidiate),
            0x04 => self.mov_ba(),
            0x05 => self.add_b(immidiate),
            0x06 => self.in_b(),
            0x07 => self.mov_b(immidiate),
            0x09 => self.out_b(),
            0x0b => self.out(immidiate),
            0x0c => self.jnc(immidiate),
            0x0f => self.jmp(immidiate),
            _ => {
                log::error!("Unknown OpeCode! {}", opecode as u8);
            }
        }
    }

    fn add_a(&mut self, immidiate: u8) {
        let new_val = self.a.get() + immidiate;
        if self.carry.is_overflow(new_val) {
            self.carry.set(1);
        } else {
            self.carry.set(0);
        }
        self.a.set(new_val);
    }

    fn mov_ab(&mut self) {
        let new_val = self.b.get();
        self.a.set(new_val);
        self.carry.set(0);
    }

    fn in_a(&mut self, immidiate: u8) {
        self.a.set(immidiate);
        self.carry.set(0);
    }

    fn mov_a(&mut self, immidiate: u8) {
        self.a.set(immidiate);
        self.carry.set(0);
    }

    fn mov_ba(&mut self) {
        let new_val = self.a.get();
        self.b.set(new_val);
        self.carry.set(0);
    }

    fn add_b(&mut self, immidiate: u8) {
        let new_val = self.b.get() + immidiate;
        if self.carry.is_overflow(new_val) {
            self.carry.set(1);
        } else {
            self.carry.set(0);
        }
        self.b.set(new_val);
    }

    fn in_b(&mut self) {
        let new_val = self.input.get();
        self.b.set(new_val);
        self.carry.set(0);
    }

    fn mov_b(&mut self, immidiate: u8) {
        self.b.set(immidiate);
        self.carry.set(0);
    }

    fn out_b(&mut self) {
        let new_val = self.b.get();
        self.output.set(new_val);
        self.carry.set(0);
    }

    fn out(&mut self, immidiate: u8) {
        self.output.set(immidiate);
        self.carry.set(0);
    }

    fn jmp(&mut self, immidiate: u8) {
        self.pc.set(immidiate);
        self.carry.set(0);
    }

    fn jnc(&mut self, immidiate: u8) {
        if self.carry.get() == 0 {
            self.pc.set(immidiate);
            self.carry.set(0);
        }
        self.carry.set(0);
    }
}
