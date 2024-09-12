use std::fs;

fn main() {
    let contents = fs::read_to_string("multiple-rules.yaml")
        .expect("Should have been able to read the file");

    let rule_file: RuleFile = serde_yaml_ng::from_str::<RuleFile>(&contents).unwrap();

    println!("{:#?}", rule_file)
}

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
