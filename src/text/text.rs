pub mod matcher {
    #[allow(unused_imports)]
    #[allow(unused_assignments)]
    use crate::cli::cli::cli::CliOptions;
    use std::io::{BufReader, Read};
    use std::fs::File;
    use yansi::{Paint, Color, Style};

    pub fn find_matches(args: &CliOptions, reader: &mut BufReader<File>) {
        let mut text = String::new();
        let mut count: u32 = 0;
        for _ in reader.read_to_string(&mut text) {

        }
        let pattern = &args.pattern;

        if args.line_number == true {
            find_pattern_and_print_line_number(&text, pattern.parse().unwrap());
        }

        if args.count == true {
            if pattern.len() > 1 {
                count = count_words(&text, pattern.parse().unwrap());
            } else {
                count = count_chars(&text, pattern.parse().unwrap());
            }
            println!("{}", count);
        }
        if text.contains(&args.pattern) && args.count == false {
            println!("{}", text);
        }
    }

    fn find_pattern_and_print_line_number(text: &String, pattern: String) {
        if text.len() > 1 {
            find_patterns_for_words(&text, pattern);
        } else {

        }
        std::process::exit(0);
    }

    fn find_patterns_for_words(text: &String, pattern: String) {
        let mut line_count = 0;
        let mut count = 0;
        text.lines().for_each(|line| {
            line_count += 1;
            let temp_line = line.trim();
            temp_line.split_ascii_whitespace().for_each(|word| {
                if pattern.eq(word) {
                    count += 1;
                    println!("{}:{}", Paint::green(line_count), Paint::black(word));
                }
            })
        });
        let style = Style::new(Color::Blue).bold().blink();
        println!("{} The pattern `{}` appeared {} times in this search", Paint::masked("🎉🎉"), style.paint(pattern), count);
    }

    fn count_chars(text: &String, pattern: String) -> u32 {
        let mut count = 0;
        let b: char = pattern.char_indices().next().unwrap().1;
        text.chars().for_each(|c| {
            if c.eq(&b) {
                count += 1;
            }
        });

        return count;
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