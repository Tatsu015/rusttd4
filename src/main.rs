use std::fs::File;
use std::io;
use std::io::prelude::*;
use tokio::time::{sleep, Duration};
extern crate env_logger as logger;
extern crate log;
use std::env;

mod register;
mod rom;

struct Cpu {
    pc: register::Register,
    carry: register::Register,
    a: register::Register,
    b: register::Register,
    rom: rom::Rom,
    input: register::Register,
    output: register::Register,
}

impl Cpu {
    fn show(&self) {
        let s = format!("{:0>4b}", self.output.get());
        let s = s.replace("0", "□").replace("1", "■");
        println!("{}", s);
    }

    fn fetch(&self) -> u8 {
        log::debug!("Fetch program");
        let ins = self.rom.get_instruction(self.pc.get());
        return ins;
    }

    fn pc_up(&mut self) {
        let pc = self.pc.get();
        self.pc.set(pc + 1);
        log::debug!("PC count up")
    }

    fn decode(&self, instruction: u8) -> (u8, u8) {
        log::debug!("Decode instruction");
        let imm = instruction & 0x0f;
        let ope = (instruction & 0xf0) >> 4;
        return (ope, imm);
    }

    fn execute(&mut self, opecode: u8, immidiate: u8) {
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
        self.carry.set(1);
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

struct Emulator {
    cpu: Cpu,
}

impl Emulator {
    fn show(&self) {
        self.cpu.show();
    }

    fn run(&mut self) {
        self.show();
        let instruction = self.cpu.fetch();
        self.cpu.pc_up();
        let decoded = self.cpu.decode(instruction);
        self.cpu.execute(decoded.0, decoded.1);
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "info");
    logger::init();

    let mut f = File::open("example/bin/ramentimer")?;
    let mut program = Vec::new();
    f.read_to_end(&mut program)?;

    let rom = rom::Rom { program };
    let cpu = Cpu {
        pc: register::Register::new(),
        carry: register::Register::new(),
        a: register::Register::new(),
        b: register::Register::new(),
        rom: rom,
        input: register::Register::new(),
        output: register::Register::new(),
    };
    let mut emulator = Emulator { cpu };

    loop {
        emulator.run();
        sleep(Duration::from_millis(1000)).await;
    }
}
