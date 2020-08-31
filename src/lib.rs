pub mod matcher {
    use std::io::{BufReader, BufRead, Read};
    use std::fs::File;
    use crate::cli::Cli;

    pub fn find_matches(args: &Cli, reader: &mut BufReader<File>) {
        let mut line = String::new();
        let mut count: u32 = 0;
        for _ in reader.read_to_string(&mut line) {
            if line.contains(&args.pattern) && args.count == true {
                println!("{}", count);
                count += 1;
            }
            if line.contains(&args.pattern) && args.count == false {
                println!("{}", line);
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
        #[structopt(parse(from_flag), short="-c", long="--count", help = "print only a count of selected lines per FILE")]
        pub(crate) count: bool
    }
}
