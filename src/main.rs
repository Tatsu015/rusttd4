mod compiler;
mod cpu;
mod emulator;
mod opecode;
mod register;
mod rom;

extern crate env_logger as logger;
extern crate log;

use clap::{Parser, Subcommand};
use std::env;

use compiler::Compiler;
use emulator::Emulator;

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap(about = "emulate td4 cpu")]
    Emulate {
        #[clap(long)]
        program: String,

        #[clap(long, default_value_t = 100)]
        clock: u64,
    },
    #[clap(about = "compile binary file for td4 cpu emulator")]
    Compile {
        #[clap(long)]
        src: String,

        #[clap(long)]
        dst: String,
    },
}

#[derive(Parser, Debug)]
#[clap(version = "1.0", about = "td4 cpu emulator")]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() {
    env::set_var("RUST_LOG", "info");
    logger::init();

    let cli = Args::parse();
    match cli.subcmd {
        SubCommand::Emulate { program, clock } => {
            let mut e = Emulator::new(&program);
            e.run(clock);
        }
        SubCommand::Compile { src, dst } => {
            let c = Compiler::new();
            c.compile(&src)
        }
    }
}
