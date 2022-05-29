fn main() {
    if let Err(err) = csvfldselector::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
