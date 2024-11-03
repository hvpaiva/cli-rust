use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;

/// A simple command line tool to count lines, words, [chars] and bytes in a file or standard input. (See chars with the -m option)
///
/// The wcr utility displays the number of lines, words, chars and bytes contained in each input file, or standard input (if no file is specified) to the
/// standard output. A line is defined as a string of characters delimited by a ⟨newline⟩ character. Characters beyond the final ⟨newline⟩ character
/// will not be included in the line count.
///
/// A word is defined as a string of characters delimited by white space characters. White space characters are the set of characters for which the
/// iswspace(3) function returns true. If more than one input file is specified, a line of cumulative counts for all the files is displayed on a
/// separate line after the output for the last file.
///
/// The options below may be used to select which counts are printed, always in
/// the following order: line, word, character, byte.
///
/// If no options are specified, the default is to print line, word and byte. Same as -lwm.
///
/// This is a rust implementation of the `wc` command line tool.
#[derive(Debug, Parser)]
#[command(about, version, author, long_about)]
struct Args {
    /// The file(s) to count. If no file is specified or if file is -, read from standard input.
    #[arg(name = "FILE", default_value = "-")]
    files: Vec<String>,
    /// Print the number of lines in each input file.
    #[arg(short, long)]
    lines: bool,
    /// Print the number of words in each input file.
    #[arg(short, long)]
    words: bool,
    /// Print the number of bytes in each input file.
    #[arg(short = 'c', long)]
    bytes: bool,
    /// Print the number of characters in each input file.
    #[arg(short = 'm', long)]
    chars: bool,
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

impl FileInfo {
    fn new() -> Self {
        FileInfo {
            num_lines: 0,
            num_words: 0,
            num_bytes: 0,
            num_chars: 0,
        }
    }

    fn sum(&mut self, other: &FileInfo) {
        self.num_lines += other.num_lines;
        self.num_words += other.num_words;
        self.num_bytes += other.num_bytes;
        self.num_chars += other.num_chars;
    }
}

fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        num_bytes += line_bytes;
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_chars,
        num_bytes,
    })
}

fn show(filename: &String, info: &FileInfo, args: &Args) -> String {
    let mut output = String::new();

    if args.lines {
        output.push_str(&format!("{:>8}", info.num_lines));
    }
    if args.words {
        output.push_str(&format!("{:>8}", info.num_words));
    }
    if args.chars {
        output.push_str(&format!("{:>8}", info.num_chars));
    }
    if args.bytes {
        output.push_str(&format!("{:>8}", info.num_bytes));
    }

    let input = if filename == "-" {
        ""
    } else {
        &format!(" {filename}")
    };
    output.push_str(input);

    output
}

fn run(mut args: Args) -> Result<()> {
    if [args.lines, args.words, args.bytes, args.chars]
        .iter()
        .all(|&b| !b)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let mut total = FileInfo::new();

    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                let count = count(file)?;
                println!("{}", show(filename, &count, &args));
                total.sum(&count);
            }
        }
    }

    if args.files.len() > 1 {
        let label = "total".to_string();
        println!("{}", show(&label, &total, &args));
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use io::Cursor;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_count() {
        let text = "I don't want the world.\nI just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_show() {
        for fixture in fixtures_show() {
            let output = show(&fixture.filename, &fixture.info, &fixture.args);
            assert_eq!(output, fixture.expected);
        }
    }

    fn fixtures_show() -> Vec<ShowFixture> {
        let show_all = ShowFixture {
            filename: "test.txt".to_string(),
            args: Args {
                lines: true,
                words: true,
                bytes: true,
                chars: true,
                files: vec![],
            },
            info: FileInfo {
                num_lines: 2,
                num_words: 10,
                num_chars: 48,
                num_bytes: 58,
            },
            expected: "       2      10      48      58 test.txt".to_string(),
        };
        let show_lwb = ShowFixture {
            filename: "test.txt".to_string(),
            args: Args {
                lines: true,
                words: true,
                bytes: true,
                chars: false,
                files: vec![],
            },
            info: FileInfo {
                num_lines: 2,
                num_words: 10,
                num_chars: 48,
                num_bytes: 58,
            },
            expected: "       2      10      58 test.txt".to_string(),
        };
        let show_l = ShowFixture {
            filename: "-".to_string(),
            args: Args {
                lines: true,
                words: false,
                bytes: false,
                chars: false,
                files: vec![],
            },
            info: FileInfo {
                num_lines: 2,
                num_words: 10,
                num_chars: 48,
                num_bytes: 58,
            },
            expected: "       2".to_string(),
        };

        vec![show_all, show_lwb, show_l]
    }

    struct ShowFixture {
        filename: String,
        args: Args,
        info: FileInfo,
        expected: String,
    }
}
