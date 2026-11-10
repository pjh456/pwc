use clap::Parser;
use anyhow::Result;
use std::option::Option;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // Count lines of input
    #[arg(short, long)]
    pub lines: bool, 

    // Count words of input
    #[arg(short, long)]
    pub words: bool,

    // Count bytes of input
    #[arg(short, long)]
    pub bytes: bool,

    // Input files
    #[arg(value_name = "FILE")]
    pub files: Vec<String>, 
}

impl Cli {
    pub fn parse_args() -> Result<Self> { Ok(Self::try_parse()?) }

    pub fn inputs(&self) -> Vec<Option<&str>> { vec![] }
}
