# PJH's wc CLI

Rewrite `wc` cli in Rust, a beginner project for me to learn Rust.

## Usage

Same as `wc` itself.

Each line print selected counts and automatically making alignment among lines.

You can use it in 3 ways:

1. Provide one or more path, and it will count every files: `pwc file1.txt file2.txt`
2. Provide no path and use stdin for input: `... | pwc`
3. Mix path and stdin inputs, and use `-` stands for stdin inputs: `pwc file1.txt - file2.txt -`

> Of course you can count nothing from stdin using `-` after first time, but it's a upstream action inherited from `wc`, and there's no reason to fix it.

### Options

Options are composable, and multiple options follwing orders to print each one.

If option is missing, default option groups are `-l`, `-w` and `-c`.

| Order | Option                    | Effect                                         |
| ----- | ------------------------- | ---------------------------------------------- |
| 1     | `-l`, `--lines`           | Print `\n` counts                              |
| 2     | `-w`, `--words`           | Print words counts (split by ASCII whitespace) |
| 3     | `-m`, `--chars`           | Print character counts                         |
| 4     | `-c`, `--bytes`           | Print bytes counts                             |
| 5     | `-L`, `--max-line-length` | Print character length of max line             |

### Arguments

| Argument         | Effect                                                          |
| ---------------- | --------------------------------------------------------------- |
| `-h`, `--help`   | Print help text                                                 |
| `--version`      | Print CLI version                                               |
| `--total=auto`   | Automatically print total counts when receiving multiple inputs |
| `--total=always` | Always print total counts after print counts of all inputs      |
| `--total=only`   | Only print total counts                                         |
| `--total=never`  | Count every input without print total counts                    |

> Noting that `-h` is not supported in `wc` but supported here, that's a default action by clap, and I don't want to fix it since it's harmless :)

## License

MIT.
