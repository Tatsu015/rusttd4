use std::io;

use tokio::time::{sleep, Duration};
extern crate env_logger as logger;
extern crate log;
use std::env;

mod cpu;
mod register;
mod rom;

use crate::cpu::Cpu;
use crate::register::Register;
use crate::rom::Rom;

struct Emulator {
    cpu: cpu::Cpu,
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

    let rom = Rom::new("example/bin/ramentimer")?;
    let input = Register::new();
    let output = Register::new();
    let cpu = Cpu::new(rom, input, output);
    let mut emulator = Emulator { cpu };

    loop {
        emulator.run();
        sleep(Duration::from_millis(100)).await;
    }
}
