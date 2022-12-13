/*
    BIOS can be started by a specifying it specifically
*/

use std::fs::read_to_string;
use clap::{self, Parser};
use phantasm_ir::spectre_ir::Instructions;
use rei_vm::vm::{MainMemory, new_main_memory_2_gigabytes, MainProcessingUnit, new_mpu};

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    Compile,
    Run,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

    match args.action {
        Action::Compile => {
            // if compile BIOS, compile the firmware...
            println!("Compiling BIOS...")
        },
        Action::Run => {
            println!("Welcome to Rei VM!");
            println!("Running the default BIOS...");

            // read the ELF from firmware/bin or "directly"?
            // let bios = read_to_string("firmware/bin").expect("Couldn't read the BIOS software");
            // deserialise into Vec<SpectreInstruction>
            // let program: Vec<SpectreInstruction> = bios;
            let main_memory = new_main_memory_2_gigabytes();
            let mpu = new_mpu();

            // let bios = std::fs::read("firmware/bin").unwrap();
            // let bios = efficient_assemble(bios);
            // basically, bincode deserialize
            // let program: Instructions = bincode::deserialize(&bios).unwrap();

            // REPL or runner
            println!("Done!");
        }
    }

    Ok(())
}
