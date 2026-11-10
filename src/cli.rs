use anyhow::Result;
use clap::Parser;
use std::{
    option::Option,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // Count lines of input
    #[arg(short, long)]
    pub lines: bool,

    // Count words of input
    #[arg(short, long)]
    pub words: bool,

    // Count characters of input
    #[arg(short, long)]
    pub chars: bool,

    // Count bytes of input
    #[arg(short, long)]
    pub bytes: bool,

    // Count max length of line
    #[arg(short = 'L', long)]
    pub max_line_len: bool,

    // Input files
    #[arg(value_name = "FILE")]
    pub files: Vec<PathBuf>,
}

impl Cli {
    pub fn parse_args() -> Result<Self> {
        Ok(Self::try_parse()?)
    }

    pub fn inputs(&self) -> Vec<Option<&Path>> {
        if self.files.is_empty() {
            vec![None]
        } else {
            self.files
                .iter()
                .map(|p| {
                    if p.as_os_str() == "-" {
                        None
                    } else {
                        Some(p.as_path())
                    }
                })
                .collect()
        }
    }
}
