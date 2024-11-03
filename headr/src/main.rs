use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    usize,
};

use anyhow::Result;
use clap::Parser;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// Print the first N lines of a FILE to standard output.
///
/// With no FILE, or when FILE is -, read standard input.
///
/// With more than one FILE, precede each with a header giving the file name.
///
/// This is a rust implementation of the `head` command.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
struct Args {
    #[arg(name = "FILE", default_value = "-")]
    /// The input file(s) to use, or `-` for stdin
    files: Vec<String>,
    #[arg(
        value_name = "NUM",
        short = 'n',
        long,
        default_value = "10",
        conflicts_with = "bytes"
    )]
    /// The number of lines to print.
    lines: u64,
    #[arg(value_name = "NUM", short = 'c', long, conflicts_with = "lines")]
    /// The number of bytes to print.
    bytes: Option<u64>,
    #[arg(short, long, conflicts_with = "verbose")]
    /// Never print headers giving file names.
    quiet: bool,
    #[arg(short, long, conflicts_with = "quiet")]
    /// Always print headers giving file names.
    verbose: bool,
}

struct Modification {
    files_count: usize,
    lines: u64,
    bytes: Option<u64>,
    quiet: bool,
    verbose: bool,
}

fn headr(buff: Box<dyn BufRead>, modification: &Modification) {
    if modification.bytes.is_some() {
        let bytes = modification.bytes.unwrap();
        print_by_number_of_bytes(buff, bytes);
    } else {
        print_by_number_of_lines(buff, modification.lines);
    }
}

fn print_by_number_of_lines(mut buff: Box<dyn BufRead>, lines: u64) {
    let mut line = String::new();
    for _ in 0..lines {
        let bytes = buff.read_line(&mut line).unwrap();

        if bytes == 0 {
            break;
        }

        print!("{line}");
        line.clear();
    }
}

fn print_by_number_of_bytes(mut buff: Box<dyn BufRead>, bytes: u64) {
    let mut buffer = vec![0; bytes as usize];
    let bytes_read = buff.read(&mut buffer).unwrap();
    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
}

fn include_header(filename: &str, file_index: usize, modification: &Modification) {
    if !modification.quiet && (modification.verbose || modification.files_count > 1) {
        println!(
            "{}==> {filename} <==",
            if file_index > 0 { "\n" } else { "" }
        );
    }
}

fn run(args: Args) -> Result<()> {
    let m = Modification {
        files_count: args.files.len(),
        lines: args.lines,
        bytes: args.bytes,
        quiet: args.quiet,
        verbose: args.verbose,
    };
    for (i, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(buff) => {
                include_header(filename, i, &m);
                headr(buff, &m);
            }
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
