use anyhow::Result;
use pwc::cli::{Cli, TotalWhen};
use pwc::count::count;
use pwc::input::Input;
use pwc::output::print_results;
use pwc::{Col, Label, Row, Stats};
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

    let print_total = match cli.total {
        TotalWhen::Auto => inputs.len() > 1,
        TotalWhen::Always | TotalWhen::Only => true,
        TotalWhen::Never => false,
    };

    for input in inputs {
        let row = process_one(input, &cli.columns())?;
        total.stats.merge(&row.stats);

        if cli.total != TotalWhen::Only {
            rows.push(row);
        }
    }

    if print_total {
        rows.push(total);
    }

    print_results(cli, &rows);

    Ok(())
}

fn process_one<'a>(input: &Option<&'a Path>, cols: &[Col]) -> Result<Row<'a>> {
    let mut inner = Input::open(*input)?;

    Ok(Row {
        stats: count(&mut inner, cols)?,
        label: match &input {
            Some(p) => Label::File(p),
            None => Label::Stdin,
        },
    })
}
