use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use anyhow::{anyhow, Result};
use clap::{arg, command, Parser};

/// A command-line tool to check repeated lines in a file.
///
/// If the file is not specified or is "-", it reads from standard input.
///
/// if the output file is not specified, it writes to standard output.
///
///
/// This is a Rust implementation of the `uniq` command in Unix-like systems,
/// with the difference that the lines do not need to be adjacent to be considered repeated.
#[derive(Debug, Parser)]
#[command(about, version, author, long_about)]
struct Args {
    /// The input file to read from. If not specified or is "-", it reads from standard input.
    #[arg(default_value = "-")]
    in_file: String,
    /// The output file to write to. If not specified, it writes to standard output.
    out_file: Option<String>,
    /// Precede each output line with the count of the number times the line occurred in the input, followed by a space.
    #[arg(short, long)]
    count: bool,
    /// Output a single copy of each line that is repeated in the input.
    #[arg(short = 'd', long, conflicts_with = "unique")]
    repeated: bool,
    /// Case-insensitive comparison of lines.
    #[arg(short, long)]
    ignore_case: bool,
    /// Only output lines that are not repeated in the input.
    #[arg(short, long, conflicts_with = "repeated")]
    unique: bool,
    /// Consider lines to be repeated only if they are adjacent.
    #[arg(short, long, conflicts_with = "unique", conflicts_with = "repeated")]
    adjacent: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let mut file = open(&args.in_file).map_err(|e| anyhow!("{}: {e}", args.in_file))?;
    let mut out_file: Box<dyn Write> = match &args.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };
    let mut line = String::new();
    let mut count = HashMap::new();
    let mut previous = String::new();
    let mut count_adj: u64 = 0;
    let mut print = |num: u64, text: &str| {
        if num > 0 {
            if args.count {
                write!(out_file, "{num:>4} {text}").unwrap();
            } else {
                write!(out_file, "{text}").unwrap();
            }
        };
    };

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if args.ignore_case {
            line = line.to_lowercase();
        }
        if args.adjacent {
            if line.trim_end() != previous.trim_end() {
                print(count_adj, &previous);
                previous = line.clone();
                count_adj = 0;
            }

            count_adj += 1;
        } else {
            count
                .entry(line.clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(0);
        }

        line.clear();
    }
    if args.adjacent {
        print(count_adj, &previous);
        return Ok(());
    }
    for (line, counter) in count {
        if args.repeated && counter == 0 {
            continue;
        }
        if args.unique && counter > 0 {
            continue;
        }
        if args.count {
            write!(out_file, "{:4} ", counter + 1)?;
        }
        write!(out_file, "{line}")?;
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
