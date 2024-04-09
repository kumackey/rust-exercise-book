use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::App;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;

                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                        continue;
                    }

                    println!("{}", line);
                }
            }
        }
    }

    Ok(())
}

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub number_lines: bool,
    pub number_nonblank_lines: bool,
}

const NUMBER_NONBLANK: &str = "number_nonblank";

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Kyohei Kumaki <dummy@dummy.com>")
        .about("Rust cat")
        .arg(clap::Arg::with_name("files")
                 .value_name("FILES")
                 .help("Input files")
                 .multiple(true)
                 .default_value("-"),
        )
        .arg(clap::Arg::with_name("number")
            .short("n")
            .long("number")
            .help("Number all output lines")
            .takes_value(false)
            .conflicts_with(NUMBER_NONBLANK)
        )
        .arg(clap::Arg::with_name(NUMBER_NONBLANK)
            .short("b")
            .long("number-nonblank")
            .help("Number nonempty output lines")
            .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}
