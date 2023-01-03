use std::fs::File;
use std::io::{BufReader, self};
use std::{error::Error, io::BufRead};
use std::ops::Range;
use clap::{Parser, command, crate_authors, crate_version, ArgGroup};
use csv::StringRecord;
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
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
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
        files: args.files.clone(),
        delimiter: get_delimiter(&args)?,
        extract: get_extract(&args)? })
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
        Err(Box::<dyn Error>::from("No positions were specified".to_string()))
    }

}

fn parse_pos(range: &str) -> MyResult<PositionList> {
    range
        .split(',')
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
    match match_obj.as_str().parse::<usize>() {
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

    for filename in &config.files {
        match open(filename) {
            Ok(mut file) => run_file(&mut file, &config),
            Err(e) => eprintln!("{}: {}", filename, e),
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run_file(file: &mut Box<dyn BufRead>, config: &Config) {
    if let Extract::Fields(positions) = &config.extract {
        extract_fields_from_file(file, positions, config);
    } else {
        file.lines().flatten().for_each(|line| {
            run_line(&line, config);
        });
    }
}

fn extract_fields_from_file(file: &mut Box<dyn BufRead>, positions: &Vec<Range<usize>>,
    config: &Config) {

    let mut delim_str = String::new();
    delim_str.push(config.delimiter as char);

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(config.delimiter)
        .from_reader(file);

    if let Ok(header) = reader.headers() {
        let header_fields = extract_fields(header, positions);
        println!("{}", header_fields.join(&delim_str));
    }

    for record in reader.records().flatten() {
        let fields = extract_fields(&record, positions);
        println!("{}", fields.join(&delim_str));
    }

}

fn extract_fields(record: &StringRecord, field_positions: &[Range<usize>]) -> Vec<String> {

    let extracted: Vec<String> = field_positions
        .iter()
        .flat_map(|rng| { fields_in_range(record, rng) })
        .collect();

    extracted
}

fn fields_in_range(record: &StringRecord, range: &Range<usize>) -> Vec<String> {

    let mut ret: Vec<String> = vec![];

    for i in range.start..range.end {
        if let Some(field) = record.get(i) {
            ret.push(field.to_string());
        }
    }

    ret
}

fn run_line(line: &str, config: &Config) {

    let extracted = match &config.extract {
        Extract::Chars(positions) => extract_chars(line, positions),
        Extract::Bytes(positions) => extract_bytes(line, positions),
        _ => return,
    };

    println!("{}", extracted);
}

fn extract_chars(line: &str, char_positions: &[Range<usize>]) -> String {

    let chars: Vec<char> = line.chars().collect();

    char_positions
        .iter()
        .map(|rng| { chars_in_range(&chars, rng) })
        .collect::<Vec<String>>()
        .join("")

}

fn chars_in_range(chars: &Vec<char>, range: &Range<usize>) -> String {
    let l = chars.len();
    let s = range.start;
    let e = range.end;

    if s < e && e <= l {
        let mut ret = String::new();
        for &ch in chars[s..e].iter() {
            ret.push(ch);
        }
        ret
    } else {
        "".to_string()
    }
}

fn extract_bytes(line: &str, byte_positions: &[Range<usize>]) -> String {

    let bytes: Vec<u8> = line.bytes().collect();

    byte_positions
        .iter()
        .map(|rng| { bytes_in_range(&bytes, rng) })
        .collect::<Vec<String>>()
        .join("")
}

fn bytes_in_range(bytes: &[u8], range: &Range<usize>) -> String {
    let l = bytes.len();
    let s = range.start;
    let e = range.end;

    if s < e && e <= l {
        String::from_utf8_lossy(&bytes[s..e]).to_string()
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod unit_tests {
    use csv::StringRecord;

    use super::{parse_pos, extract_chars, extract_bytes, extract_fields};

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

    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars("", &[0..1]), "".to_string());
        assert_eq!(extract_chars("ábc", &[0..1]), "á".to_string());
        assert_eq!(extract_chars("ábc", &[0..1, 2..3]), "ác".to_string());
        assert_eq!(extract_chars("ábc", &[0..3]), "ábc".to_string());
        assert_eq!(extract_chars("ábc", &[2..3, 1..2]), "cb".to_string());
        assert_eq!(
            extract_chars("ábc", &[0..1, 1..2, 4..5]),
            "áb".to_string()
        );
    }

    #[test]
    fn test_extract_bytes() {
        assert_eq!(extract_bytes("ábc", &[0..1]), "�".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
        assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
        assert_eq!(extract_bytes("ábc", &[3..4, 2..3]), "cb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2, 5..6]), "á".to_string());
    }

    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2]), &["Sham"]);
        assert_eq!(
            extract_fields(&rec, &[0..1, 2..3]),
            &["Captain", "12345"]
        );
        assert_eq!(extract_fields(&rec, &[0..1, 3..4]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2, 0..1]), &["Sham", "Captain"]);
    }

}
