// kknd2-cplc-parser
// Copyright (c) 2024 Matthew Costa <ucosty@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::error::Error;
use clap::Parser;

mod creature_library;
mod cplc;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    filename: String,

    #[arg(short, long, default_value = "Creature.klb")]
    creature_library: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let creature_library = creature_library::parse(&cli.creature_library)?;

    cplc::parse(&cli.filename, &creature_library)?;

    Ok(())
}
