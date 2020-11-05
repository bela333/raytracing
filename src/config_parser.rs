use std::fs::{File, self};

use clap::{App, Arg};
use serde_derive::*;
#[derive(Deserialize)]
pub struct TomlConfig{

}

impl TomlConfig {
    pub fn read_file(filename: &str) -> Self{
        let f = fs::read_to_string(filename).unwrap();
        toml::from_str(f.as_str()).unwrap()
    }
}

pub struct Config{
    pub toml: TomlConfig
}

impl Config{
    pub fn get() -> Self{
        let matches = App::new("Path Tracer")
            .version("0.0.1")
            .author("bela333 <b3kstudio@gmail.com>")
            .arg(Arg::with_name("debug")
                .short("d")
                .long("debug"))
            .arg(Arg::with_name("scene")
                .index(1)
                .required(true)).get_matches();
        let file_path = matches.value_of("scene").unwrap();
        Self{
            toml: TomlConfig::read_file(file_path)
        }
    }
}