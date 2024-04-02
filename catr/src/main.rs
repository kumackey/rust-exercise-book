use std::error::Error;

use clap::App;

fn main() -> MyResult<()> {
    if let Err(err) = get_args().and_then(run) {
        eprintln!("{}", err);
        std::process::exit(1);
    }

    println!("Hello, world!");
    Ok(())
}

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

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr").version("0.1.0").author("Kyohei Kumaki <dummy@dummy.com>").about("Rust cat").get_matches();
}
