use std::fs::File;
use std::io;
use std::io::prelude::*;
use tokio::time::{sleep, Duration};
extern crate env_logger as logger;
extern crate log;
use std::env;

const REGISTER_CAPACITY: u8 = 0x0f;
struct Rom {
    program: Vec<u8>,
}

impl Rom {
    fn get_instruction(&self, adress: u8) -> u8 {
        return self.program[adress as usize];
    }
}

struct Cpu {
    pc: u8,
    carry: u8,
    a: u8,
    b: u8,
    rom: Rom,
    input: u8,
    output: u8,
}

impl Cpu {
    fn show(&self) {
        let s = format!("{:0>4b}", self.output);
        let s = s.replace("0", "□").replace("1", "■");
        println!("{}", s);
    }

    fn fetch(&self) -> u8 {
        log::debug!("Fetch program");
        let ins = self.rom.get_instruction(self.pc);
        return ins;
    }

    fn pc_up(&mut self) {
        self.pc += 1;
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
        let new_val = self.a + immidiate;
        if new_val > REGISTER_CAPACITY {
            self.carry = 1
        } else {
            self.carry = 0
        }
        self.a = new_val
    }

    fn mov_ab(&mut self) {
        self.a = self.b;
        self.carry = 0;
    }

    fn in_a(&mut self, immidiate: u8) {
        self.a = immidiate;
        self.carry = 0;
    }

    fn mov_a(&mut self, immidiate: u8) {
        self.a = immidiate;
        self.carry = 0;
    }

    fn mov_ba(&mut self) {
        self.b = self.a;
        self.carry = 0;
    }

    fn add_b(&mut self, immidiate: u8) {
        let new_val = self.b + immidiate;
        if new_val > REGISTER_CAPACITY {
            self.carry = 1;
        } else {
            self.carry = 0;
        }
        self.b = new_val;
    }

    fn in_b(&mut self) {
        self.b = self.input;
        self.carry = 0;
    }

    fn mov_b(&mut self, immidiate: u8) {
        self.b = immidiate;
        self.carry = 0;
    }

    fn out_b(&mut self) {
        self.output = self.b;
        self.carry = 0;
    }

    fn out(&mut self, immidiate: u8) {
        self.output = immidiate;
        self.carry = 0;
    }

    fn jnc(&mut self, immidiate: u8) {
        self.pc = immidiate;
        self.carry = 0;
    }

    fn jmp(&mut self, immidiate: u8) {
        if self.carry == 0 {
            self.pc = immidiate;
            self.carry = 0;
        }
        self.carry = 0;
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

    let mut f = File::open("example/bin/led")?;
    let mut program = Vec::new();
    f.read_to_end(&mut program)?;

    let rom: Rom = Rom { program };
    let cpu = Cpu {
        pc: 0,
        carry: 0,
        a: 0,
        b: 0,
        rom: rom,
        input: 0,
        output: 0,
    };
    let mut emulator = Emulator { cpu };

    loop {
        emulator.run();
        sleep(Duration::from_millis(100)).await;
    }
}
