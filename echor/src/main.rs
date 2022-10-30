use clap::{Command, Arg, ArgAction};

fn main() {
    let matches = Command::new("echor")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Thomas Bollmeier <developer@thomas-bollmeier.de>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .long("no-newline")
                .help("Do not print newline")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text = matches.get_many::<String>("text")
        .unwrap_or_default()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    
    let omit_newline = if let Some(omit_nl) = matches.get_one::<bool>("omit_newline") {
        *omit_nl
    } else {
        false
    };

    let ending = if omit_newline { 
        ""
    } else {
        "\n"
    };

    print!("{}{}", text, ending);
 
}
