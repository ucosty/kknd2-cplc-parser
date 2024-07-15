use std::error::Error;
use std::fmt::Display;
use std::io::{Read, Seek};

use byteorder::ReadBytesExt;
use clap::Parser;

use crate::parser::parse_cplc;

mod units;
mod parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    filename: String,
}


fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    parse_cplc(&cli.filename)?;
    Ok(())
}
