use anyhow::Result;
use clap::{command, Parser, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// A simple command line tool for searching files, directories and links
///
/// This is a Rust implementation of the `find` command in Unix
#[derive(Debug, Parser)]
#[command(version, author)]
struct Args {
    /// The paths to search
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,
    /// The regular expression names to search
    #[arg(value_name = "NAME", short = 'n', long = "name", num_args = 0..)]
    names: Vec<Regex>,
    /// The type of the search
    #[arg(value_name = "TYPE", short = 't', long = "type", num_args = 0..)]
    types: Vec<Types>,
}

#[derive(ValueEnum, Debug, Eq, PartialEq, Clone)]
enum Types {
    /// A file
    #[value(name = "f")]
    File,
    /// A directory
    #[value(name = "d")]
    Dir,
    /// A link
    #[value(name = "l")]
    Link,
}

fn run(args: Args) -> Result<()> {
    let type_filter = |entry: &DirEntry| {
        args.types.is_empty()
            || args.types.iter().any(|t| match t {
                Types::Link => entry.file_type().is_symlink(),
                Types::Dir => entry.file_type().is_dir(),
                Types::File => entry.file_type().is_file(),
            })
    };

    let name_filter = |entry: &DirEntry| {
        args.names.is_empty()
            || args
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };
    let result_to_option = |res| match res {
        Err(e) => {
            eprintln!("{e}");
            None
        }
        Ok(entry) => Some(entry),
    };

    for path in &args.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(result_to_option)
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }
    Ok(())
}
