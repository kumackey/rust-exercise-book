fn main() {
    if let Err(err) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}