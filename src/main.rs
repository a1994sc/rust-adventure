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
const IMAGES_KEY: &str = "images";

fn main() {
    let flag: Flags = Flags::parse();
    let s: String = std::fs::read_to_string(&flag.package).expect("could not read file");

    let mut docs: Vec<yaml_rust::Yaml> = YamlLoader::load_from_str(&s).unwrap();

    for doc in docs.iter_mut() {
        let list: yaml_rust::Yaml = yaml_rust::Yaml::String(LIST_KEY.to_string());

        if let Some(package) = doc.as_hash().filter(|hash| hash.contains_key(&list)) {
            if let Some(pkgs) = package[&list].as_hash() {
                for key in pkgs.keys() {
                    let chart: yaml_rust::Yaml = yaml_rust::Yaml::String(CHART_KEY.to_string());
                    let version: yaml_rust::Yaml = yaml_rust::Yaml::String(VERSION_KEY.to_string());
                    let images: yaml_rust::Yaml = yaml_rust::Yaml::String(IMAGES_KEY.to_string());

                    if let Some(v) = pkgs[&key]
                        .as_hash()
                        .filter(|hash| hash.contains_key(&chart))
                    {
                        println!("{:?}", v[&chart]);
                    }

                    if let Some(v) = pkgs[&key]
                        .as_hash()
                        .filter(|hash| hash.contains_key(&version))
                    {
                        println!("{:?}", v[&version]);
                    }

                    if let Some(v) = pkgs[&key]
                        .as_hash()
                        .filter(|hash| hash.contains_key(&images))
                    {
                        println!("{:?}", v[&images]);
                    }
                }
            }
        }
    }
}
