use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

use anyhow::Result;
use clap::{arg, Command};

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn catr(buff: Box<dyn BufRead>, number_lines: bool, number_non_blank: bool) {
    let mut num = 0;
    for (i, l) in buff.lines().enumerate() {
        let line = l.unwrap();
        if number_lines {
            println!("{:>6}\t{}", i + 1, line);
        } else if number_non_blank {
            if line.is_empty() {
                println!();
            } else {
                num += 1;
                println!("{num:>6}\t{line}")
            }
        } else {
            println!("{line}");
        }
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {filename}: {e}"),
            Ok(buff) => catr(buff, args.number_lines, args.number_non_blank),
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_non_blank: bool,
}

fn get_args() -> Args {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Highlander Paiva <contact@hvpaiva.dev>")
        .about("catr is a cat clone written in Rust")
        .args([
            arg!(-n --number "Print line numbers").conflicts_with("number-nonblank"),
            arg!(-b --"number-nonblank" "Print line numbers for non-blank lines")
                .conflicts_with("number"),
            arg!([FILE] ... "Input file(s) to read").default_value("-"),
        ])
        .get_matches();

    Args {
        files: matches.get_many("FILE").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number"),
        number_non_blank: matches.get_flag("number-nonblank"),
    }
}
