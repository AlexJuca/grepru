pub mod cli {
    use structopt::StructOpt;
    use std::path::PathBuf;

    #[derive(StructOpt)]
    #[structopt(name = "grepru", about = "print lines that match patterns", author = "Alexandre Juca<corextechnologies@gmail.com>")]
    pub struct CliOptions {
        #[structopt(required_unless = "version")]
        pub(crate) pattern: String,
        #[structopt(parse(from_os_str), required_unless = "version")]
        pub path: PathBuf,
        #[structopt(parse(from_flag), short="-c", long="--count", help = "print only a count of selected lines per FILE")]
        pub(crate) count: bool,
        #[structopt(parse(from_flag), short="-n", long="--line-number", help = "Prefix each line of output with the 1-based line number within its input file.")]
        pub(crate) line_number: bool
    }
}
