use clap::Parser;
use std::io;

use tokio::time::{sleep, Duration};
extern crate env_logger as logger;
extern crate log;
use std::env;

mod cpu;
mod emulator;
mod register;
mod rom;

use crate::emulator::Emulator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    program_path: String,

    #[arg(short, long, default_value_t = 100)]
    clock: u64,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    env::set_var("RUST_LOG", "info");
    logger::init();

    let mut emulator = Emulator::new(&args.program_path);
    let clock = args.clock;

    loop {
        emulator.run();
        sleep(Duration::from_millis(clock)).await;
    }
}
