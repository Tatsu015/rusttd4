use tokio::time::{sleep, Duration};

type Register = u8;

struct Rom {}

impl Rom {
    fn get_immidiate(&self) -> u8 {
        return 0;
    }
}

struct Cpu {
    pc: Register,
    carry: Register,
    a: Register,
    b: Register,
    rom: Rom,
    input: Register,
    output: Register,
}

impl Cpu {
    fn fetch(&self) -> u8 {
        println!("Fetch program");
        let imm = self.rom.get_immidiate();
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
    fn run(&self) {
        self.cpu.fetch();
        self.cpu.pc_up();
        self.cpu.decode();
        self.cpu.execute();
    }
}

#[tokio::main]
async fn main() {
    let cpu = Cpu {
        pc: 0,
        a: 0,
        b: 0,
        carry: 0,
        input: 0,
        output: 0,
    };
    let emulator = Emulator { cpu };

    loop {
        emulator.run();
        sleep(Duration::from_millis(1000)).await;
    }
}
