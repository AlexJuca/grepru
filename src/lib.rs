pub mod matcher {
    #[allow(unused_imports)]
    use std::io::{BufReader, BufRead, Read};
    use std::fs::File;
    use crate::cli::Cli;

    pub fn find_matches(args: &Cli, reader: &mut BufReader<File>) {
        let mut line = String::new();
        for _ in reader.read_to_string(&mut line) {
            if line.contains(&args.pattern) {
                println!("|> {}", line);
            } else {
                println!("The pattern `{}` was not found in file {}", &args.pattern, std::env::args().nth(2).unwrap());
            }
        }
    }
}

pub mod cli {
    use structopt::StructOpt;
    use std::path::PathBuf;

    #[derive(StructOpt)]
    #[structopt(name = "grepru", about = "print lines that match patterns", author = "Alexandre Juca<corextechnologies@gmail.com>")]
    pub struct Cli {
        #[structopt(required_unless = "version")]
        pub(crate) pattern: String,
        #[structopt(parse(from_os_str), required_unless = "version")]
        pub path: PathBuf,
    }
}
