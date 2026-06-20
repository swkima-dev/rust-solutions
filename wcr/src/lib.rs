use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(short, long)]
    lines: bool,

    #[arg(short, long)]
    words: bool,

    #[arg(short('c'), long)]
    bytes: bool,

    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut buf = String::new();

    loop {
        let line_bytes = file.read_line(&mut buf)?;
        if line_bytes == 0 {
            break;
        }

        num_bytes += line_bytes;
        num_lines += 1;
        num_words += buf.split_whitespace().count();
        num_chars += buf.chars().count();
        buf.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{FileInfo, count};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}

pub fn run(mut args: Args) -> Result<()> {
    if [args.lines, args.words, args.bytes, args.chars]
        .iter()
        .all(|x| x == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let mut total = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };

    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let info = count(file)?;
                if args.lines {
                    print!("{:>8}", info.num_lines);
                }
                if args.words {
                    print!("{:>8}", info.num_words);
                }
                if args.bytes {
                    print!("{:>8}", info.num_bytes);
                }
                if args.chars {
                    print!("{:>8}", info.num_chars);
                }
                if filename != "-" {
                    print!(" {}", filename);
                }
                println!("");
                total.num_lines += info.num_lines;
                total.num_words += info.num_words;
                total.num_bytes += info.num_bytes;
                total.num_chars += info.num_chars;
            }
        }
    }
    if &args.files.len() > &1 {
        if args.lines {
            print!("{:>8}", total.num_lines);
        }
        if args.words {
            print!("{:>8}", total.num_words);
        }
        if args.bytes {
            print!("{:>8}", total.num_bytes);
        }
        if args.chars {
            print!("{:>8}", total.num_chars);
        }
        println!(" total",);
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
