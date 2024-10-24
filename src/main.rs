use clap::Parser;
use yaml_rust::{YamlEmitter, YamlLoader};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Flags {
    /// The path to the file to read
    #[arg(long)]
    path: std::path::PathBuf,
}

fn main() {
    let flag: Flags = Flags::parse();
    let s: String = std::fs::read_to_string(&flag.path).expect("could not read file");

    let docs: Vec<yaml_rust::Yaml> = YamlLoader::load_from_str(&s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc: &yaml_rust::Yaml = &docs[0];

    // Debug support
    println!("{:?}", doc);

    // Dump the YAML object
    let mut out_str: String = String::new();
    {
        let mut emitter: YamlEmitter<'_> = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
}
