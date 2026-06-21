use anyhow::{Result, anyhow};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    #[arg(short, long)]
    count: bool,
}

pub fn run(args: Args) -> Result<()> {
    let mut file = open(&args.in_file).map_err(|e| anyhow!("{}: {}", args.in_file, e))?;

    let mut line = String::new();
    let mut prev = String::new();
    let mut dupl_count: usize = 0;

    let mut out_file: Box<dyn Write> = match &args.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: usize, text: &str| -> Result<()> {
        if count > 0 {
            if args.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if line.trim_end() != prev.trim_end() {
            print(dupl_count, &prev);
            prev = line.clone();
            dupl_count = 0;
        }
        dupl_count += 1;
        line.clear();
    }
    print(dupl_count, &prev);
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
