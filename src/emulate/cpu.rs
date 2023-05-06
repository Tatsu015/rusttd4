use crate::opecode::Opecode;

use super::{register::Register, rom::Rom};

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
            rom,
            input,
            output,
        };
        return cpu;
    }

    pub fn get_out(&self) -> u8 {
        return self.output.get();
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
        let op = Opecode::u8_to_ope(opecode).unwrap();
        match op {
            Opecode::AddA => self.add_a(immidiate),
            Opecode::MovAB => self.mov_ab(),
            Opecode::InA => self.in_a(immidiate),
            Opecode::MovA => self.mov_a(immidiate),
            Opecode::MovBA => self.mov_ba(),
            Opecode::AddB => self.add_b(immidiate),
            Opecode::InB => self.in_b(),
            Opecode::MovB => self.mov_b(immidiate),
            Opecode::OutB => self.out_b(),
            Opecode::Out => self.out(immidiate),
            Opecode::Jnc => self.jnc(immidiate),
            Opecode::Jmp => self.jmp(immidiate),
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
