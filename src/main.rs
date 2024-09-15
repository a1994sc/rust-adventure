extern crate schema_lib;

use clap::{Parser, Subcommand};
use schema_lib::linkage::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Function to combine two positive int, order matters
    Pair { a: u32, b: u32 },
    /// Function to separate any positive int into two numbers
    Separate { id: u32 },
}

// const ZARF_SCHEMA: &str = include_str!("../schema/zarf.schema.json");

fn main() {
    let args: Args = Args::parse();

    match &args.cmd {
        Commands::Pair { a, b } => {
            println!("{:?}", pair(Decoded { a: *a, b: *b }));
        }
        Commands::Separate { id } => {
            println!("{:?}", separate(Encoded { id: *id }));
        }
    }
}
