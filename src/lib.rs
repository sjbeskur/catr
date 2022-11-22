use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(&config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open: {}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (num, result) in file.lines().enumerate() {
                    let line = result?;
                    if config.number_lines {
                        println!("{:<6}\t{}", num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:<6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

//#[warn(dead_code)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.0.3")
        .author("Sam Beskur <sam.beskur@gmail.com>")
        .about("cats and dogs living together..")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s) to read")
                .num_args(1..3)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .num_args(0)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .num_args(0),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap_or_default()
            .map(|v| v.to_owned())
            .collect::<Vec<_>>(),
        number_lines: matches.get_one::<bool>("number").unwrap().to_owned(), //.contains_id("number"),
        number_nonblank_lines: matches
            .get_one::<bool>("number_nonblank")
            .unwrap()
            .to_owned(), //contains_id("number_nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
