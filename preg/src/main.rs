use clap::Parser;
use std::{fs::File, io::{self, BufRead, BufReader}, process};
use atty::Stream;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    /// The pattern to search for
    #[arg(short, long, value_name = "PATTERN", aliases = ["p"])]
    pattern: String,

    /// The path to the file to search in (use "-" for stdin)
    #[arg(short, long, value_name = "FILE", aliases = ["f"])]
    filename: Option<String>,

    /// Case insensitive search
    #[arg(short, long, aliases = ["i"])]
    ignore_case: bool,

    /// Show line numbers
    #[arg(short = 'n', long, aliases = ["l"])]
    line_numbers: bool,

    /// Only show count of matching lines
    #[arg(short, long, aliases = ["c"])]
    count: bool,

    /// Show only matching part of the line
    #[arg(short = 'o', long, aliases = ["m"])]
    only_matching: bool,

    /// Invert the match (show non-matching lines)
    #[arg(short = 'v', long)]
    invert_match: bool,

    /// Colorize matched text
    #[arg(long, value_name = "WHEN", default_value = "auto")]
    color: String,
}

enum ColorWhen {
    Always,
    Never,
    Auto,
}

fn main() {
    let args = CLI::parse();

    let color_when = match args.color.as_str() {
        "always" => ColorWhen::Always,
        "never"  => ColorWhen::Never,
        "auto"   => ColorWhen::Auto,
        _        => ColorWhen::Auto,
    };
    let use_color = match color_when {
        ColorWhen::Always => true,
        ColorWhen::Never => false,
        ColorWhen::Auto => atty::is(Stream::Stdout),
    };

    let pattern = if args.ignore_case {
        args.pattern.to_lowercase()
    } else {
        args.pattern.clone()
    };

    let reader: Box<dyn BufRead> = match &args.filename {
        Some(name) if name != "-" => {
            match File::open(name) {
                Ok(f) => Box::new(BufReader::new(f)),
                Err(e) => {
                    eprintln!("Error opening file '{}': {}", name, e);
                    process::exit(1);
                }
            }
        }
        _ => {
            if atty::is(Stream::Stdin) {
                eprintln!("No file specified and no input piped to stdin.");
                process::exit(1);
            } else {
                Box::new(BufReader::new(io::stdin()))
            }
        }
    };


    let mut matches = 0;

    for (line_no, result) in reader.lines().enumerate() {
        let Ok(line) = result else {
            eprintln!("Error reading line {}", line_no+1);
            continue;
        };

        let haystack = if args.ignore_case { line.to_lowercase() } else { line.clone() };
        let found = haystack.contains(&pattern);
        let is_match = if args.invert_match { !found } else { found };
        if is_match {
            matches += 1;

            if !args.count {
                if args.only_matching {
                    for (start_idx, _) in haystack.match_indices(&pattern) {
                        let slice = &line[start_idx..start_idx+pattern.len()];
                        if args.line_numbers {
                            print!("{}:", line_no + 1);
                        }
                        if use_color {
                            print!("\x1b[31m{}\x1b[0m", slice); // red
                        } else {
                            print!("{}", slice);
                        }
                        println!();
                    }
                } else {
                    if args.line_numbers {
                        print!("{}:", line_no + 1);
                    }

                    if use_color && found && !args.invert_match {
                        // highlight all pattern occurrences in line
                        let mut last = 0;
                        for (start, _) in haystack.match_indices(&pattern) {
                            let orig_start = if args.ignore_case {
                                haystack[..start].len()
                            } else { start };
                            let orig_end = orig_start + pattern.len();
                            print!("{}", &line[last..orig_start]);
                            print!("\x1b[31m{}\x1b[0m", &line[orig_start..orig_end]);
                            last = orig_end;
                        }
                        println!("{}", &line[last..]);
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }

    if args.count {
        println!("{matches}");
    }
}