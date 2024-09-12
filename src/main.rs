// use std::fs;

// fn main() {
//     let contents = fs::read_to_string("multiple-rules.yaml")
//         .expect("Should have been able to read the file");

//     let rule_file: RuleFile = serde_yaml_ng::from_str::<RuleFile>(&contents).unwrap();

//     println!("{:#?}", rule_file)
// }

use serde::{Deserialize, Serialize};

// struct to hold the entire rule file
#[derive(Debug, Serialize, Deserialize)]
struct RuleFile {
    rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rule {
    id: String,
    severity: String,
    languages: Vec<String>,
    dummy: Option<String>,
}
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short = 'n', long)]
    name: String,

    /// Number of times to greet
    #[arg(short = 'c', long, default_value_t = 1)]
    count: u8,
    // /// Version of the Big Bang to base the zarf.yaml file off
    // #[arg(short = 'v', long = "bb-version")]
    // bb_version: String,
}

fn main() {
    let zarf_schema: &str = include_str!("../schema/zarf.schema.json");
    let args: Args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    println!("{}", zarf_schema.len());
}
