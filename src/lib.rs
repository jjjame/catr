use clap::{Arg, ArgAction, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_non_blank_line: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let args = Command::new("catr")
        .version("0.1")
        .author("jjjame")
        .about("rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("files to cat")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("number")
                .value_name("NUMBER")
                .help("display line numbers")
                .short('n')
                .num_args(0),
        )
        .arg(
            Arg::new("number_non_blank")
                .value_name("NUMBER_NON_BLANK")
                .help("display line numbers non blank")
                .short('b')
                .num_args(0),
        )
        .get_matches();

    let files = args
        .get_many::<String>("files")
        .ok_or("arg not found")?
        .filter_map(|s: &String| s.parse::<String>().ok())
        .collect();

    Ok(Config {
        files,
        number_lines: args.get_flag("number"),
        number_non_blank_line: args.get_flag("number_non_blank"),
    })
}
