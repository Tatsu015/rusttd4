use crate::cpu::Cpu;
use crate::register::Register;
use crate::rom::Rom;

pub struct Emulator {
    cpu: Cpu,
}

impl Emulator {
    pub fn new() -> Emulator {
        let rom = Rom::new("example/bin/ramentimer");
        let input = Register::new();
        let output = Register::new();
        let cpu = Cpu::new(rom, input, output);

        return Emulator { cpu };
    }

    pub fn run(&mut self) {
        self.show();

        let instruction = self.cpu.fetch();
        self.cpu.pc_up();
        let decoded = self.cpu.decode(instruction);
        self.cpu.execute(decoded.0, decoded.1);
    }

    fn show(&self) {
        self.cpu.show();
    }
}
