use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Kyohei Kumaki <dummy@dummy.com>")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Show word count")
                .takes_value(false),
            )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Show byte count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Show character count")
                .takes_value(false)
                .conflict_with("bytes")
            )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Show line count")
                .takes_value(false),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars")

    if [lines, words, bytes, chars].iter().all(|&x| !x) {
        // 全てfalseだったら、デフォルトでline, word, byteをtrueにする
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.is_present("lines"),
        words: matches.is_present("words"),
        bytes: matches.is_present("bytes"),
        chars: matches.is_present("chars"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
