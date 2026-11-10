use clap::Parser;

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
