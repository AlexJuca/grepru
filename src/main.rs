use std::fs::File;
use std::io::BufReader;
use grepru::matcher;
use grepru::cli::Cli;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    let content = File::open(&args.path).unwrap_or_else( |err| {
        panic!("Failed to read path {}", err);
    });

    let mut reader = BufReader::new(content);
    matcher::find_matches(&args, &mut reader)
}


