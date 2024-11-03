use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = "Highlander Paiva <[email protected]>",
    version,
    about = "An implementation of echo command"
)]
struct Args {
    #[arg(required = true, help = "The text to be printed", num_args = 1..)]
    text: Vec<String>,
    #[arg(short = 'n', help = "Do not output the trailing newline")]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" },
    )
}
