use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_non_blank_line: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut i = 1;

                for line in file.lines() {
                    let current_line = line?;
                    let is_empty = current_line.trim().is_empty();
                    if config.number_non_blank_line {
                        if is_empty {
                            println!("{}", current_line);
                        } else {
                            println!("{}    {}", i, current_line);
                            i += 1;
                        }
                    } else if config.number_lines {
                        println!("{}    {}", i, current_line);
                        i += 1;
                    } else {
                        println!("{}", current_line);
                    }
                }
            }
        }
    }
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
                .action(ArgAction::Append)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .value_name("NUMBER")
                .help("Number lines")
                .short('n')
                .long("number")
                .num_args(0),
        )
        .arg(
            Arg::new("number_nonblank")
                .value_name("NUMBER_NONBLANK")
                .help("Number non-blank lines")
                .short('b')
                .long("number-nonblank")
                .num_args(0),
        )
        .get_matches();

    let files = args
        .get_many::<String>("files")
        .ok_or("argument files not found")?
        .filter_map(|s: &String| s.parse::<String>().ok())
        .collect();

    Ok(Config {
        files,
        number_lines: args.get_flag("number"),
        number_non_blank_line: args.get_flag("number_nonblank"),
    })
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
