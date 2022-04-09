use std::process;

use clap::Parser;
use replace::Args;

fn main() {
    let args = Args::parse();

    if let Err(e) = args.replace(args.get_target()) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
