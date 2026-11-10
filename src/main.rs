use anyhow::Result;
use pwc::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse_args()?;

    println!("{} = {}", "-l", cli.lines);
    println!("{} = {}", "-w", cli.words);
    println!("{} = {}", "-b", cli.bytes);
    println!("{} = {:?}", "FILES", cli.files);

    Ok(())
}
