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
            let decoded = self.cpu.decode(instruction);

            // pc count up before execute, because execute maybe change pc.
            self.cpu.pc_up();
            self.cpu.execute(decoded.0, decoded.1);
            let out = self.cpu.get_out();

            Self::show(out);

            sleep(Duration::from_millis(clock));
        }
    }

    fn show(out: u8) {
        println!("\x1b[H");
        let s = format!("{:0>4b}", out);
        let s = s.replace("0", "□").replace("1", "■");
        println!("\r{}", s);
    }
}
