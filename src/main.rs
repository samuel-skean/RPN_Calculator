mod calculator;
mod lexing;
use std::{fs::File, io::BufReader, path::PathBuf};

use calculator::calculate;
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    /// The input file to read from.
    input_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let input = BufReader::new(File::open(args.input_file).expect("Unable to open file."));

    let tokens = lexing::run(input);

    let result = calculate(tokens.into_iter().map(|i| i.token));

    match result {
        Ok(n) => println!(
            "The result of the calculation was:\n\
             {}",
            n
        ),
        Err(msg) => panic!(
            "Uh oh! There was an error:\n\
             {}",
            msg
        ),
    }
}
