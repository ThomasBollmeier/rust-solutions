use clap::{Command, Arg};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Thomas Bollmeier <developer@thomas-bollmeier.de>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let mut text: String = "".to_string();
    let mut texts = matches.values_of("text").unwrap();
    
    while let Some(part) = texts.next() {
        if !text.is_empty() {
            text += " ";
        }
        text += part;
    }

    let ending = if matches.is_present("omit_newline") { 
        ""
    } else {
        "\n"
    };

    print!("{}{}", text, ending);
 
}
