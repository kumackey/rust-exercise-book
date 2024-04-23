use std::error::Error;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub lines: usize,
    pub bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .about("Rust head")
        .author("Kyohei Kumaki <dummy@dummy.com>")
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let lines = matches
        .value_of("lines")
        .map(|n| parse_positive_int(n))
        .unwrap_or(Ok(10))?;
    let bytes = matches
        .value_of("bytes")
        .map(|n| n.parse().unwrap());

    Ok(Config { files, lines, bytes })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_positive_int(s: &str) -> MyResult<usize> {
    match s.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(s.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
