extern crate env_logger as logger;
extern crate log;

use clap::{Parser, Subcommand};
use std::env;
use std::path::Path;

use rusttd4::compile::compiler::Compiler;
use rusttd4::emulate::emulator::Emulator;

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

            let extension = Path::new(&src).extension().unwrap();
            if extension != "td4asm" {
                panic!("extension need to td4asm");
            }
            if dst == "" {
                let same_dst = String::from(Path::new(&src).file_stem().unwrap().to_str().unwrap());
                c.compile(&src, &same_dst)
            } else {
                c.compile(&src, &dst)
            }
        }
    }
}
