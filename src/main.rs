
mod text;
mod cli;

use std::fs::File;
use std::io::BufReader;
use structopt::StructOpt;
use cli::cli::cli::CliOptions;

fn main() {
    let args = CliOptions::from_args();

    let content = File::open(&args.path).unwrap_or_else( |err| {
        panic!("{}", err);
    });

    let mut reader = BufReader::new(content);
    text::text::matcher::find_matches(&args, &mut reader)
}


