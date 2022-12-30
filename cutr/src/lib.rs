use std::error::Error;
use std::ops::Range;
use clap::{Parser, command, crate_authors, crate_version, ArgGroup};
use regex::{Regex, Match};

pub type MyResult<T> = Result<T, Box<dyn Error>>;
pub type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug, Parser)]
#[command(
    author = crate_authors!("\n"),
    version = crate_version!(),
    about = "Rust version of cut"
)]
#[command(group(
    ArgGroup::new("mode")
        .args(["field_positions","byte_positions", "char_positions"])
))]
struct ConfigArgs {
    #[arg(
        value_name = "FILE",
        help = "Input file(s)",
        default_value = "-",
        num_args = 1..,
    )]
    files: Vec<String>,

    #[arg(
        short = 'd',
        long = "delim",
        value_name = "DELIMITER",
        default_value = "\t",
    )]
    delimiter: String,

    #[arg(
        short = 'f',
        long = "fields",
        value_name = "FIELDS",
        help = "Selected fields",
    )]
    field_positions: Option<String>,

    #[arg(
        short = 'b',
        long = "bytes",
        value_name = "BYTES",
        help = "Selected bytes",
    )]
    byte_positions: Option<String>,

    #[arg(
        short = 'c',
        long = "chars",
        value_name = "CHARS",
        help = "Selected characters",
    )]
    char_positions: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    _files: Vec<String>,
    _delimiter: u8,
    _extract: Extract,
}

pub fn get_config() -> MyResult<Config> {
    let args = ConfigArgs::parse();
    config_args_into_config(args)
}

fn config_args_into_config(args: ConfigArgs) -> MyResult<Config> {

    let all_positions = vec![
        &args.field_positions,
        &args.byte_positions,
        &args.char_positions,
    ];

    if all_positions.iter().all(|pos_opt| { pos_opt.is_none() }) {
        return Err(Box::<dyn Error>::from("Must have --fields, --bytes, or --chars"));
    }

    Ok(Config {
        _files: args.files.clone(),
        _delimiter: get_delimiter(&args)?,
        _extract: get_extract(&args)? })
}

fn get_delimiter(args: &ConfigArgs) -> MyResult<u8> {

    let bytes = args.delimiter.as_bytes();

    if bytes.len() != 1 {
        let err = format!("--delim \"{}\" must be a single byte", args.delimiter);
        return Err(Box::<dyn Error>::from(err.as_str()));
    }

    Ok(bytes[0])
}

fn get_extract(args: &ConfigArgs) -> MyResult<Extract> {

    if let Some(positions) = &args.field_positions {
        Ok(Extract::Fields(parse_pos(positions)?))
    } else if let Some(positions) = &args.byte_positions {
        Ok(Extract::Bytes(parse_pos(positions)?))
    } else if let Some(positions) = &args.char_positions {
        Ok(Extract::Chars(parse_pos(positions)?))
    } else {
        Err(Box::<dyn Error>::from(format!("No positions were specified")))
    }

}

fn parse_pos(range: &str) -> MyResult<PositionList> {
    range
        .split(",")
        .map(interval_to_range)
        .collect()
}

fn interval_to_range(interval: &str) -> MyResult<Range<usize>> {

    let re = Regex::new("^(\\d+)(-(\\d+))?$")?;

    let captures= match re.captures(interval) {
        Some(caps) => caps,
        None => return create_interval_error(interval),
    };

    let start = match captures.get(1) {
        Some(m) => parse_index(&m)?,
        None => return create_interval_error(interval),
    };

    if start < 1 {
        return create_interval_error(&format!("{}", start));
    }

    let end = captures
        .get(3)
        .map(|m| { parse_index(&m) })
        .transpose()?;

    if let Some(end) = end {
        if start < end {
            Ok((start - 1)..end)
        } else {
            create_range_error(
                &format!("First number in range ({}) must be lower than second number ({})", start, end))
        }
    } else {
        Ok((start - 1)..start)
    }
}

fn parse_index(match_obj: &Match) -> MyResult<usize> {
    match usize::from_str_radix(match_obj.as_str(), 10) {
        Ok(idx) => Ok(idx),
        Err(error) => Err(Box::<dyn Error>::from(format!("{}", error))),
    }
}

fn create_interval_error(interval: &str) -> MyResult<Range<usize>> {
    create_range_error(&format!("illegal list value: \"{}\"", interval))
}

fn create_range_error(message: &str) -> MyResult<Range<usize>> {
    Err(Box::<dyn Error>::from(message))
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}


#[cfg(test)]
mod unit_tests {
    use super::parse_pos;

    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(parse_pos("").is_err());

        // Zero is an error
        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        let res = parse_pos("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        // A leading "+" is an error
        let res = parse_pos("+1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"+1\"",
        );

        let res = parse_pos("+1-2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"+1-2\"",
        );

        let res = parse_pos("1-+2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"1-+2\"",
        );

        // Any non-number is an error
        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1-a");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"1-a\"",
        );

        let res = parse_pos("a-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"a-1\"",
        );

        // Wonky ranges
        let res = parse_pos("-");
        assert!(res.is_err());

        let res = parse_pos(",");
        assert!(res.is_err());

        let res = parse_pos("1,");
        assert!(res.is_err());

        let res = parse_pos("1-");
        assert!(res.is_err());

        let res = parse_pos("1-1-1");
        assert!(res.is_err());

        let res = parse_pos("1-1-a");
        assert!(res.is_err());

        // First number must be less than second
        let res = parse_pos("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }

}
