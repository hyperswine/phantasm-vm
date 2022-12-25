/*
    BIOS can be started by a specifying it specifically
*/

use std::fs::read_to_string;
use clap::{self, Parser};

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

            // REPL or runner
            println!("Done!");
        }
    }

    Ok(())
}
