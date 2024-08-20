use crate::EntryType::*;
use regex::Regex;
use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
pub enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    pub paths: Vec<String>,
    pub names: Vec<Regex>,
    pub entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
    .version("0.1.0")
    .author("Kyohei Kumaki <example-kumaki@example.com>")
    .about("Rust find command")
    .arg(Arg::with_name("path")
        .value_name("PATH")
        .help("Search path")
        .required(true)
        .default_value(".")
        .multiple(true)
        .index(1))
    .arg(Arg::with_name("name")
        .value_name("NAME")
        .help("File name pattern")
        .takes_value(true)
        .multiple(true)
        .long("name"))
    .arg(Arg::with_name("type")
        .value_name("TYPE")
        .help("Entry type")
        .takes_value(true)
        .multiple(true)
        .long("type")
        .possible_values(&["f", "d", "l"]))
    .get_matches();

    let paths = matches.values_of("path").unwrap().map(ToString::to_string).collect();
    let names = matches.values_of("name").map(|values| {
        values.map(|value| Regex::new(value).map_err(|e| format!("{}", e)).unwrap()).collect()
    }).unwrap_or_default();
    let entry_types = matches.values_of("type").map(|values| {
        values.map(|value| match value {
            "f" => File,
            "d" => Dir,
            "l" => Link,
            // helpによるエラーを表示
            _ => unreachable!(),
        }).collect()
    }).unwrap_or_default();

    Ok(Config {
        paths,
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}