use std::thread::sleep;
use std::time::Duration;

use crate::cpu::Cpu;
use crate::register::Register;
use crate::rom::Rom;

pub struct Emulator {
    cpu: Cpu,
}

impl Emulator {
    pub fn new(program_path: &str) -> Emulator {
        let rom = Rom::new(program_path);
        let input = Register::new();
        let output = Register::new();
        let cpu = Cpu::new(rom, input, output);

        return Emulator { cpu };
    }

    pub fn run(&mut self, clock: u64) {
        println!("\x1b[2J");
        loop {
            let instruction = self.cpu.fetch();
            self.cpu.pc_up();
            let decoded = self.cpu.decode(instruction);
            self.cpu.execute(decoded.0, decoded.1);
            sleep(Duration::from_millis(clock));
            self.show();
        }
    }

    fn show(&self) {
        println!("\x1b[H");
        let out = self.cpu.get_out();
        let s = format!("{:0>4b}", out);
        let s = s.replace("0", "□").replace("1", "■");
        println!("\r{}", s);
    }
}
