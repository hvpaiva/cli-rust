use std::ops::Range;

use anyhow::bail;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, version, author)]
struct Args {
    /// Files to process
    #[arg(default_value = "-")]
    files: Vec<String>,
    /// Field delimiter
    #[arg(short, long, default_value = "\t")]
    delimiter: String,

    #[command(flatten)]
    extract: ArgExtract,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct ArgExtract {
    /// Select only these fields
    #[arg(short, long)]
    fields: Option<String>,
    /// Select only these characters
    #[arg(short, long)]
    chars: Option<String>,
    /// Select only these bytes
    #[arg(short, long)]
    bytes: Option<String>,
}

type Extraction = Vec<Range<usize>>;

pub fn parse_extraction(_range: &str) -> anyhow::Result<Extraction> {
    unimplemented!()
}

#[derive(Debug)]
pub enum Extract {
    Fields(Extraction),
    Bytes(Extraction),
    Chars(Extraction),
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let delim_bytes = args.delimiter.as_bytes();
    if delim_bytes.len() > 1 {
        bail!(
            r#"delimiter "{}" is invalid. It must be a single byte"#,
            args.delimiter
        );
    }
    let delimiter = *delim_bytes.first().unwrap();
    println!("{delimiter}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest(
        input,
        expected_msg,
        case("", r#"illegal list value: ""#),
        case("0", r#"illegal list value: "0""#),
        case("0-1", r#"illegal list value: "0""#),
        case("+1", r#"illegal list value: "+1""#),
        case("+1-2", r#"illegal list value: "+1-2""#),
        case("1-+2", r#"illegal list value: "1-+2""#),
        case("a", r#"illegal list value: "a""#),
        case("1,a", r#"illegal list value: "a""#),
        case("1-a", r#"illegal list value: "1-a""#),
        case("a-1", r#"illegal list value: "a-1""#),
        case("-", ""),
        case(",", ""),
        case("1,", ""),
        case("1-", ""),
        case("1-1-1", ""),
        case("1-1-a", "")
    )]
    fn test_parse_extraction_illegal_values(input: &str, expected_msg: &str) {
        let res = parse_extraction(input);
        assert!(res.is_err());
        if !expected_msg.is_empty() {
            assert_eq!(res.unwrap_err().to_string(), expected_msg);
        }
    }

    #[rstest(
        input,
        expected_msg,
        case(
            "1-1",
            "First number in range (1) must be lower than second number (1)"
        ),
        case(
            "2-1",
            "First number in range (2) must be lower than second number (1)"
        )
    )]
    fn test_parse_extraction_invalid_ranges(input: &str, expected_msg: &str) {
        let res = parse_extraction(input);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), expected_msg);
    }

    #[test]
    fn test_parse_extraction_failure() {
        assert!(parse_extraction("").is_err());

        let res = parse_extraction("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);

        let res = parse_extraction("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);

        // A leading "+" is an error
        let res = parse_extraction("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "+1""#,);

        let res = parse_extraction("+1-2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "+1-2""#,
        );

        let res = parse_extraction("1-+2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "1-+2""#,
        );

        // Any non-number is an error
        let res = parse_extraction("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);

        let res = parse_extraction("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);

        let res = parse_extraction("1-a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "1-a""#,);

        let res = parse_extraction("a-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a-1""#,);

        // Wonky ranges
        let res = parse_extraction("-");
        assert!(res.is_err());

        let res = parse_extraction(",");
        assert!(res.is_err());

        let res = parse_extraction("1,");
        assert!(res.is_err());

        let res = parse_extraction("1-");
        assert!(res.is_err());

        let res = parse_extraction("1-1-1");
        assert!(res.is_err());

        let res = parse_extraction("1-1-a");
        assert!(res.is_err());

        // First number must be less than second
        let res = parse_extraction("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_extraction("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );
    }
}
