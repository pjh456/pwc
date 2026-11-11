use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::{
    option::Option,
    path::{Path, PathBuf},
};

use crate::Col;

#[cfg(test)]
mod tests;

#[derive(ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum TotalWhen {
    Auto,
    Always,
    Only,
    Never,
}

macro_rules! define_cli {
    (
        $(
            $(#[$meta:meta])*
            $field:ident : $variant:ident : $short:literal / $long:literal
        ),* $(,)?
        ;
        $( $extra:tt )*
    ) => {
        #[derive(Parser, Debug)]
        #[command(version, about, long_about = None)]
        pub struct Cli {
            $(
                $(#[$meta])*
                #[arg(short = $short, long = $long)]
                pub $field: bool,
            )*
            $( $extra )*
            #[arg(value_name = "FILE")]
            pub files: Vec<PathBuf>,
        }

        impl Cli {
            pub fn columns(&self) -> Vec<Col> {
                let any  = false $(|| self.$field)*;
                if !any {
                    return vec![Col::Lines, Col::Words, Col::Bytes];
                }
                let mut cols = Vec::new();
                $(if self.$field { cols.push(Col::$variant); })*
                cols
            }
        }
    };
}

define_cli! {
    // Count lines
    lines: Lines: 'l' / "lines",
    // Count words
    words: Words: 'w' / "words",
    // Count characters
    chars: Chars: 'm' / "chars",
    // Count bytes
    bytes: Bytes : 'c' / "bytes",
    // Count line length
    max_line_len: MaxLineLen: 'L' / "max-line-length",
    ;
    #[arg(long, value_enum, default_value="auto")]
    pub total: TotalWhen,
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
