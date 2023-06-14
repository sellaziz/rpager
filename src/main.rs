#![allow(clippy::cognitive_complexity)]
mod app;
mod crossterm;
mod ui;

use crate::crossterm::run;
use argh::FromArgs;
use std::{error::Error, io, time::Duration};

const DEFAULT_INPUT: &str = "examples/input.txt";
const DEFAULT_QUERY: &str = "lorem";
/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// string to grep.
    #[argh(option)]
    pattern: Option<String>,
    /// file to reqd.
    #[argh(option)]
    filename: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    let mut stdout = io::stdout();
    let mut in_pat = DEFAULT_QUERY.to_string();
    if let Some(pattern) = cli.pattern {
        in_pat = pattern;
    }
    let mut in_fil = DEFAULT_INPUT.to_string();
    if let Some(filename) = cli.filename {
        in_fil = filename;
    }
    run(&mut stdout, tick_rate, in_pat, in_fil)?;
    Ok(())
}
