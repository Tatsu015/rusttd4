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

#[tokio::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "info");
    logger::init();
    let mut emulator = Emulator::new();

    loop {
        emulator.run();
        sleep(Duration::from_millis(100)).await;
    }
}
