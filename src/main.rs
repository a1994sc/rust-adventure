use clap::Parser;
use yaml_rust::YamlLoader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Flags {
    /// The path to the package file
    #[arg(long)]
    package: std::path::PathBuf,
}

const LIST_KEY: &str = "package-image-list";
const CHART_KEY: &str = "chart";
const VERSION_KEY: &str = "version";

fn main() {
    let flag: Flags = Flags::parse();
    let s: String = std::fs::read_to_string(&flag.package).expect("could not read file");

    let mut docs: Vec<yaml_rust::Yaml> = YamlLoader::load_from_str(&s).unwrap();

    for doc in docs.iter_mut() {
        let list: yaml_rust::Yaml = yaml_rust::Yaml::String(LIST_KEY.to_string());

        if doc.as_hash().unwrap().contains_key(&list) {
            let package: &yaml_rust::Yaml = &doc[LIST_KEY];
            let chart: yaml_rust::Yaml = yaml_rust::Yaml::String(CHART_KEY.to_string());
            let version: yaml_rust::Yaml = yaml_rust::Yaml::String(VERSION_KEY.to_string());

            for key in package.as_hash().unwrap().keys() {
                let pkg = package.as_hash().unwrap().get(key).unwrap();

                if pkg.as_hash().unwrap().contains_key(&version) {
                    println!("{:?}", pkg[VERSION_KEY].as_str().unwrap());
                }

                if pkg.as_hash().unwrap().contains_key(&chart) {
                    println!("{:?}", pkg[CHART_KEY].as_str().unwrap());
                }
            }
        }
    }
}
