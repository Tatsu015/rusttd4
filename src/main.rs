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

enum OpeCode {
    AddA = 0x00,
    MovAB = 0x01,
    InA = 0x02,
    MovA = 0x03,
    MovBA = 0x04,
    AddB = 0x05,
    InB = 0x06,
    MovB = 0x07,
    OutB = 0x09,
    Out = 0x0b,
    Jnc = 0x0c,
    Jmp = 0x0f,
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
        let imm = instruction & 0x0f;
        let ope = (instruction & 0xf0) >> 4;
        return (ope, imm);
    }

    fn execute(&self, opecode: OpeCode, immidiate: u8) {
        match opecode {
            OpeCode::AddA => self.add_a(immidiate),
            OpeCode::MovAB => self.mov_ab(immidiate),
            OpeCode::InA => self.in_a(immidiate),
            OpeCode::MovA => self.mov_a(immidiate),
            OpeCode::MovBA => self.mov_ba(immidiate),
            OpeCode::AddB => self.add_b(immidiate),
            OpeCode::InB => self.in_b(immidiate),
            OpeCode::MovB => self.mov_b(immidiate),
            OpeCode::OutB => self.out_b(immidiate),
            OpeCode::Out => self.out(immidiate),
            OpeCode::Jnc => self.jnc(immidiate),
            OpeCode::Jmp => self.jmp(immidiate),
            _ => {}
        }
        println!("Execute instruction")
    }

    fn add_a(&self, immidiate: u8) {}
    fn mov_ab(&self, immidiate: u8) {}
    fn in_a(&self, immidiate: u8) {}
    fn mov_a(&self, immidiate: u8) {}
    fn mov_ba(&self, immidiate: u8) {}
    fn add_b(&self, immidiate: u8) {}
    fn in_b(&self, immidiate: u8) {}
    fn mov_b(&self, immidiate: u8) {}
    fn out_b(&self, immidiate: u8) {}
    fn out(&self, immidiate: u8) {}
    fn jnc(&self, immidiate: u8) {}
    fn jmp(&self, immidiate: u8) {}
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
