use clap::Parser;
use std::process;

use df_inspect::{Args};

fn main() {
    let args = Args::parse();
    if let Err(e) = df_inspect::run(args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
