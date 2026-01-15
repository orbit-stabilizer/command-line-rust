use anyhow::Result;
use clap::{Parser, command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of head
struct Args {
    /// Input file(s)
    #[arg(default_value("-"), value_name("FILE"))]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        conflicts_with("bytes"),
        default_value("10"),
        long("lines"),
        short('n'),
        value_parser(clap::value_parser!(u64).range(1..)),
    )]
    lines: u64,

    /// Number of bytes
    #[arg(
        long("bytes"),
        short('c'),
        value_parser(clap::value_parser!(u64).range(1..)),
    )]
    bytes: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let num_files = args.files.len() - 1;
    for (file_num, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut file) => {
                if num_files > 1 {
                    println!("==> {filename} <==");
                }
                if let Some(num_bytes) = args.bytes {
                    let bytes: Result<Vec<_>, _> = file.bytes().take(num_bytes as usize).collect();
                    print!("{}", String::from_utf8_lossy(&bytes?));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }
                if file_num < num_files {
                    println!();
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
