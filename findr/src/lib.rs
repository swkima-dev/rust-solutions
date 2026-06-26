use crate::EntryType::*;
use anyhow::Result;
use clap::{Parser, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_name = "PATH", default_value = ".", num_args = 0..)]
    paths: Vec<String>,

    #[arg(
        short('n'),
        long("name"),
        value_name = "NAME",
        value_parser(Regex::new),
        num_args = 0..
    )]
    name: Vec<Regex>,

    #[arg(short('t'), long("type"), value_name = "TYPE", value_enum, num_args = 0..)]
    entry_types: Vec<EntryType>,
}

#[derive(Debug, Eq, PartialEq, Clone, ValueEnum)]
enum EntryType {
    #[value(name = "f")]
    File,
    #[value(name = "d")]
    Dir,
    #[value(name = "l")]
    Link,
}

pub fn run(args: Args) -> Result<()> {
    let type_filter = |entry: &DirEntry| {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type {
                Link => entry.file_type().is_symlink(),
                Dir => entry.file_type().is_dir(),
                File => entry.file_type().is_file(),
            })
    };
    let name_filter = |entry: &DirEntry| {
        args.name.is_empty()
            || args
                .name
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };
    for path in args.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if type_filter(&entry) && name_filter(&entry) {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    }
    Ok(())
}
