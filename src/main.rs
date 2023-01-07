use tokio::time::{sleep, Duration};

struct Rom {}

impl Rom {
    fn get_instruction(&self, adress: u8) -> u8 {
        return 0;
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
    fn fetch(&self) -> u8 {
        println!("Fetch program");
        let ins = self.rom.get_instruction(self.pc);
        return ins;
    }

    fn pc_up(&mut self) {
        self.carry += 1;
        println!("PC count up")
    }

    fn decode(&self, instruction: u8) -> (u8, u8) {
        println!("Decode instruction");
        return (0, 0);
    }

    fn execute(&self, opecode: u8, immidiate: u8) {
        println!("Execute instruction")
    }
}

struct Emulator {
    cpu: Cpu,
}

impl Emulator {
    fn run(&mut self) {
        let instruction = self.cpu.fetch();
        self.cpu.pc_up();
        let decoded = self.cpu.decode(instruction);
        self.cpu.execute(decoded.0, decoded.1);
    }
}

#[tokio::main]
async fn main() {
    let rom: Rom = Rom {};
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
        sleep(Duration::from_millis(1000)).await;
    }
}
