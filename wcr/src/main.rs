use clap::Parser;

fn main() {
    if let Err(e) = wcr::run(wcr::Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
