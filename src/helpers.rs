extern crate toml;

use std::fs;
use std::io::Read;

pub fn get_cargo_toml_version() -> String {
    let mut file = fs::File::open("Cargo.toml").expect("Cargo.toml not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read Cargo.toml");
    let package_config: toml::Value =
        toml::from_str(&contents).expect("Failed to parse Cargo.toml");
    let version = package_config["package"]["version"]
        .as_str()
        .expect("Failed to get version");

    return version.to_owned();
}
