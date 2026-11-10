use crate::cli::Cli;
use crate::{Col, Label, Row, Stats};

pub fn print_results(cli: &Cli, rows: &[Row]) {
    let cols = cli.columns();
    if cols.is_empty() {
        return;
    }

    let mut widths = vec![0usize; cols.len()];
    for row in rows {
        for (i, col) in cols.iter().enumerate() {
            let len = col.value(&row.stats).to_string().len();
            widths[i] = widths[i].max(len);
        }
    }

    for row in rows {
        print_one(&cols, &widths, &row.stats, &row.label);
    }
}

fn print_one(cols: &[Col], widths: &[usize], stats: &Stats, label: &Label) {
    let mut out = String::new();

    for (i, col) in cols.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{:>width$}", col.value(stats), width = widths[i]));
    }

    match label {
        Label::Stdin => {}
        Label::File(p) => {
            out.push(' ');
            out.push_str(&p.display().to_string());
        }
        Label::Total => {
            out.push(' ');
            out.push_str("total");
        }
    }

    println!("{out}");
}
