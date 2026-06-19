use anyhow::Result;
use clap::Arg;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> Result<Config> {
    let matches = clap::App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number all output lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number nonempty output lines")
                .takes_value(false)
                .conflicts_with("number_lines"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

pub fn run(config: Config) -> Result<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                let mut blank_num = 0;
                for (num, result) in reader.lines().enumerate() {
                    let l = result?;
                    let num = num + 1;
                    if config.number_lines {
                        println!("{:>6}\t{}", num, l);
                    } else if config.number_nonblank_lines {
                        if l == "" {
                            blank_num = blank_num + 1;
                            println!("{}", l);
                        } else {
                            println!("{:>6}\t{}", num - blank_num, l);
                        }
                    } else {
                        println!("{}", l);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
