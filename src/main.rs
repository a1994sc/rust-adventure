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
    Pair { a: u32, b: u32 },
}

// const ZARF_SCHEMA: &str = include_str!("../schema/zarf.schema.json");

fn main() {
    let args: Args = Args::parse();

    match &args.cmd {
        Commands::Pair { a, b } => {
            println!("The ID is {:?}", pair(Decoded { a: *a, b: *b }).id);
        }
    }

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }

    // println!("{}", ZARF_SCHEMA.len());
}
