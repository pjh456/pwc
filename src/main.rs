use anyhow::Result;
use pwc::cli::Cli;
use pwc::count::count;
use pwc::output::print_results;
use pwc::{Label, Row, Stats};
use std::fs::File;
use std::io;
use std::path::Path;

fn main() -> Result<()> {
    let cli = Cli::parse_args()?;

    let inputs = cli.inputs();

    process_all(&cli, &inputs)?;

    Ok(())
}

fn process_all(cli: &Cli, inputs: &[Option<&Path>]) -> Result<()> {
    let mut total = Row {
        stats: Stats::default(),
        label: Label::Total,
    };
    let mut rows = Vec::new();
    let multi = cli.inputs().len() > 1;

    for input in inputs {
        let row = process_one(input)?;
        total.stats.merge(row.stats);
        rows.push(row);
    }

    if multi {
        rows.push(total);
    }

    print_results(&cli, &rows);

    Ok(())
}

fn process_one<'a>(input: &Option<&'a Path>) -> Result<Row<'a>> {
    match input {
        None => {
            let stdin = io::stdin();
            let mut lock = stdin.lock();
            Ok(Row {
                stats: count(&mut lock)?,
                label: Label::Stdin,
            })
        }
        Some(path) => {
            let file = File::open(path)?;
            let mut reader = io::BufReader::new(file);
            Ok(Row {
                stats: count(&mut reader)?,
                label: Label::File(path),
            })
        }
    }
}
