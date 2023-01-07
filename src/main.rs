use tokio::time::{sleep, Duration};

struct Rom {}

impl Rom {
    fn get_immidiate(&self, adress: u8) -> u8 {
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
        let imm = self.rom.get_immidiate(self.pc);
        return imm;
    }

    fn pc_up(&mut self) {
        self.carry += 1;
        println!("PC count up")
    }

    fn decode(&self) {
        println!("Decode instruction")
    }

    fn execute(&self) {
        println!("Execute instruction")
    }
}

struct Emulator {
    cpu: Cpu,
}

impl Emulator {
    fn run(&mut self) {
        self.cpu.fetch();
        self.cpu.pc_up();
        self.cpu.decode();
        self.cpu.execute();
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
    let emulator = Emulator { cpu };

    loop {
        emulator.run();
        sleep(Duration::from_millis(1000)).await;
    }
}
