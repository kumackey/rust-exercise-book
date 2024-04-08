use std::error::Error;

use clap::App;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
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
