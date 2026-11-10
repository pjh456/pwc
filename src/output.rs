use crate::cli::Cli;
use crate::{Col, Label, Row, Stats};

fn selected_columns(cli: &Cli) -> Vec<Col> {
    let not_empty = cli.lines || cli.words || cli.chars || cli.bytes || cli.max_line_len;
    let (l, w, c, b, m) = if not_empty {
        (cli.lines, cli.words, cli.chars, cli.bytes, cli.max_line_len)
    } else {
        (true, true, false, true, false)
    };

    let mut cols = Vec::new();
    if l {
        cols.push(Col::Lines);
    }
    if w {
        cols.push(Col::Words);
    }
    if c {
        cols.push(Col::Chars);
    }
    if b {
        cols.push(Col::Bytes);
    }
    if m {
        cols.push(Col::MaxLineLen);
    }

    cols
}

pub fn print_results(cli: &Cli, rows: &[Row]) {
    let cols = selected_columns(cli);
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
