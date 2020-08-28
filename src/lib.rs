pub mod matcher {
    use std::io::{BufReader, BufRead, Read};
    use std::fs::File;
    use grepru::cli;

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
