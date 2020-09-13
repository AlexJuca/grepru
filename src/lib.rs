pub mod matcher {
    use std::io::{BufReader, BufRead, Read};
    use std::fs::File;
    use crate::cli::Cli;
    use std::ops::BitOr;
    use std::error::Error;

    pub fn find_matches(args: &Cli, reader: &mut BufReader<File>) {
        let mut text = String::new();
        let mut count: u32 = 0;
        for _ in reader.read_to_string(&mut text) {

        }
        let pattern = &args.pattern;

        if args.count == true {
            if pattern.len() > 1 {
                count = count_words(&text, pattern.parse().unwrap());
            }
            println!("{}", count);
        }
        if text.contains(&args.pattern) && args.count == false {
            println!("{}", text);
        }
    }

    fn count_words(text: &String, pattern: String) -> u32 {
        let mut count = 0;
        text.split_ascii_whitespace().for_each(|n| {
            if n.contains(&pattern) {
                count += 1;
            }
        });
        return count;
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
