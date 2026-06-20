use anyhow::{Result, anyhow};
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> Result<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .short("n")
                .long("lines")
                .help("print the first K lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .takes_value(true)
                .help("print the first K bytes")
                .conflicts_with("lines"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| {
            anyhow!(
                "error: invalid value '{e}' for \
            '--lines <LINES>': invalid digit found in string"
            )
        })?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| {
            anyhow!(
                "error: invalid value '{e}' for \
            '--bytes <BYTES>': invalid digit found in string"
            )
        })?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

pub fn run(config: Config) -> Result<()> {
    for (file_num, filename) in config.files.clone().iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut reader) => {
                if config.files.len() > 1 {
                    println!("==> {} <==", filename);
                }
                if config.bytes == None {
                    for (num, result) in reader.lines().enumerate() {
                        if num < config.lines {
                            let l = result?;
                            println!("{}", l);
                        }
                    }
                } else {
                    let mut buf = vec![0; config.bytes.unwrap()];
                    let bytes_read = reader.read(&mut buf)?;
                    print!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
                }
                if file_num + 1 < config.files.len() {
                    println!("");
                }
            }
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> Result<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(anyhow!("{}", val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
