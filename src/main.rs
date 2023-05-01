use clap::{Parser, Subcommand};

extern crate env_logger as logger;
extern crate log;
use std::env;

mod cpu;
mod emulator;
mod register;
mod rom;

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap(about = "emulate td4 cpu")]
    Emulate {
        #[clap(long)]
        program: String,

        #[clap(long, default_value_t = 1000)]
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
            println!("emulate program: {}, clock: {}", program, clock);
        }
        SubCommand::Compile { src, dst } => {
            println!("compile src: {}, dst: {}", src, dst);
        }
    }
}
