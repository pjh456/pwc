mod cli;
use clap::Parser;
use crate::cli::Cli;

fn main() {
    let cli = Cli::parse();

    println!("{} = {}", "-l", cli.lines);
    println!("{} = {}", "-w", cli.words);
    println!("{} = {}", "-b", cli.bytes);
    println!("{} = {:?}", "FILES", cli.files);
}
