use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

pub fn run(args: Args) -> MyResult<()> {
    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if args.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if args.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
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

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
