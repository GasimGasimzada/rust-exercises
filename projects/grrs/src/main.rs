#![allow(unused)]
use clap::Parser;
use log::{debug, info, warn};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

use grrs::line_contains;

#[derive(Parser)]
struct Cli {
    /// Pattern to search for
    pattern: String,

    /// Path of the file
    path: std::path::PathBuf,

    /// Show line numbers
    #[arg(short, long, default_value_t = false)]
    lines: bool,

    /// Match pattern with exact casing
    #[arg(short, long, default_value_t = false)]
    case_sensitive: bool,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    env_logger::init();
    let args = Cli::parse();

    let path = args.path.as_path();
    debug!("Provided pattern: {}", args.pattern);
    debug!("Provided path: {}", path.display());
    debug!("Show line numbers: {}", args.lines);
    debug!("Case sensitive: {}", args.case_sensitive);

    let file = File::open(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path.display(), err)))?;

    let mut reader = BufReader::new(file);

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout);

    let mut line_num: u32 = 1;
    for line_iter in reader.lines() {
        let line = line_iter.map_err(|err| CustomError(format!("{}", err)))?;

        if line_contains(&line, &args.pattern, args.case_sensitive) {
            if args.lines {
                write!(writer, "{}: ", line_num);
            }
            writeln!(writer, "{}", line);
        }

        line_num += 1;
    }

    Ok(())
}
