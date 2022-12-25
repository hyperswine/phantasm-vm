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
    Repl,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

    match args.action {
        Action::Compile => {
            // if compile BIOS, compile the firmware...
            println!("Compiling BIOS...");

            // call prei to compile firmware/bios
        },
        Action::Run => {
            println!("Welcome to Rei VM!");
            println!("Running the default BIOS...");

            println!("Done!");
        }
        Action::Repl => {
            println!("Rei REPL v0");
        }
    }

    Ok(())
}
